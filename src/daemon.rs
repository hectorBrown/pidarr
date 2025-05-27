use anyhow::{Context, Result, anyhow};
use pidarr_shared::{ConnectionState, DaemonState, Media, MediaStatus, Settings};
use qbittorrent_rust::{api_fns as qbit, core::api::QbitApi};
use radarr_api::apis as radarr;
use std::fs::{create_dir_all, remove_file};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tdarr_api::apis as tdarr;
use tokio::time::{Duration, sleep};
use walkdir::WalkDir;

#[derive(Clone)]
pub struct ApiConfigs {
    pub radarr_config: Option<radarr::configuration::Configuration>,
    pub qbit_config: Option<QbitApi>,
    pub tdarr_config: Option<tdarr::configuration::Configuration>,
}

pub async fn main(
    settings: Arc<Mutex<Settings>>,
    state: Arc<Mutex<DaemonState>>,
    api_configs: Arc<Mutex<ApiConfigs>>,
) {
    let radarr_addr: String;
    let radarr_api_key: String;
    let qbit_addr: String;
    let tdarr_addr: String;
    {
        let settings = settings.lock().unwrap();
        radarr_addr = settings.radarr_addr.clone();
        radarr_api_key = settings.radarr_api_key.clone();
        qbit_addr = settings.qbit_addr.clone();
        tdarr_addr = settings.tdarr_addr.clone();
    }
    api_configs.lock().unwrap().radarr_config =
        connect_radarr(radarr_addr, radarr_api_key, state.clone()).await;
    api_configs.lock().unwrap().qbit_config = connect_qbit(qbit_addr, state.clone()).await;
    api_configs.lock().unwrap().tdarr_config = connect_tdarr(tdarr_addr, state.clone()).await;
    loop {
        match daemon_update(settings.clone(), api_configs.clone(), state.clone()).await {
            Ok(_) => {}
            Err(e) => eprintln!("There was an error updating the daemon: {}", e),
        };
        sleep(Duration::from_secs(5)).await;
    }
}

async fn daemon_update(
    settings: Arc<Mutex<Settings>>,
    api_configs: Arc<Mutex<ApiConfigs>>,
    state: Arc<Mutex<DaemonState>>,
) -> Result<()> {
    //make sure we have all the api configs we need
    let settings = settings.lock().unwrap().clone();
    let (radarr_config, tdarr_config, mut qbit_config) =
        get_api_configs(&settings, api_configs.clone(), state.clone()).await?;
    let tdarr_root_folder = get_tdarr_root_folder(&tdarr_config).await?;

    //
    //RADARR SECTION
    //

    //loop through all the movies radarr knows about
    let radarr_movies = get_radarr_movies(&radarr_config).await?;
    let id_download_hash_map = get_radarr_grabbed_media(&radarr_config).await?;
    for movie in radarr_movies {
        // if the daemon state does not have an entry for this id, we add it
        let mut media = state.lock().unwrap().media.clone();
        if let std::collections::hash_map::Entry::Vacant(_) = media.entry(movie.id) {
            //we insert the media item here, unfinished
            let media = Media {
                title: movie.title.clone(),
                download_id: id_download_hash_map
                    .get(&movie.id)
                    .context(format!(
                        "Could not get download hash for movie {}",
                        movie.title
                    ))?
                    .to_owned(),
                radarr_path: movie.path.clone(),
                tdarr_path: None,
                download_progress: None,
                seeding_ratio: None,
                transcode_progress: None,
                status: MediaStatus::Unknown,
            };
            println!(
                "-----------\nFound movie: {}\n{:?}\n-----------",
                &movie.title, &media
            );
            state.lock().unwrap().media.insert(movie.id, media);
        }
    }

    //
    //QBIT SECTION
    //

    //grab all hashes that are in qbittorrent
    let hashes = get_qbit_torrent_hashes(&mut qbit_config).await?;
    {
        // for each item of media
        let media = state.lock().unwrap().media.clone();
        for (id, media_item) in &media {
            // match radarr's download_id with the hashes in qBittorrent
            let hash = hashes.get(&media_item.download_id).context(format!(
                "Could not find torrent hash for item {:?}",
                &media_item
            ))?;
            let props = get_qbit_torrent_props(&mut qbit_config, hash).await?;
            let progress = props.progress;
            let mut state = state.lock().unwrap();
            let media = state.media.get_mut(id).context(format!(
                "Could not get media object for item {:?}",
                &media_item
            ))?;
            //update download progress for each torrent
            media.download_progress = Some(progress);
            media.seeding_ratio = Some(props.seeding_ratio);
            if progress < 100.0 {
                media.status = MediaStatus::Downloading;
            } else if progress == 100.0 {
                //TODO: extra logic for if transcoding is finished
                media.status = MediaStatus::Transcoding;
            }
        }
    }

    //
    // TDARR SECTION
    //
    {
        // update transcode progress
        let workers_map = get_tdarr_all_workers(&tdarr_config, &tdarr_root_folder).await?;
        let mut state = state.lock().unwrap();
        for (id, media_item) in state.media.clone() {
            if let Some(radarr_path) = media_item.radarr_path.clone() {
                if let Some(prog) = workers_map.get(&radarr_path) {
                    state
                        .media
                        .get_mut(&id)
                        .context(format!(
                            "Could not get media object for item {:?}",
                            &media_item
                        ))?
                        .transcode_progress = Some(*prog);
                }
            }
        }
    }

    {
        let mut state = state.lock().unwrap();
        let tdarr_output_list = get_file_list_in(&settings.tdarr_output)?;
        for (id, media_item) in state.media.clone() {
            // if the media item is in the tdarr output list, we update its status
            if let Some(media_path) = media_item.radarr_path.clone() {
                for path in &tdarr_output_list {
                    // if the path contains the media path (extension agnostic)
                    if path.contains(
                        &Path::new(&media_path)
                            .with_extension("")
                            .to_str()
                            .context(format!(
                                "Could not parse media path for item {:?}",
                                &media_item
                            ))?
                            .to_string(),
                    ) {
                        //media is in tdarr output
                        let media = state.media.get_mut(&id).context(format!(
                            "Could not get media object for item {:?}",
                            media_path
                        ))?;
                        media.tdarr_path = Some(path.to_owned());
                        media.transcode_progress = Some(100.0);

                        if media.seeding_ratio.context(format!(
                            "Could not get seeding ratio for media {:?}",
                            &media
                        ))? >= settings.target_seeding_ratio
                        {
                            media.status = MediaStatus::Completed;
                        } else {
                            media.status = MediaStatus::Seeding;
                        }
                    }
                }
            }
        }
    }
    // now act based on status to create and remove softlinks
    //
    {
        let state = state.lock().unwrap();
        let jellyfin_links = get_file_list_in(&settings.jellyfin_input)?;
        for (_, media_item) in state.media.clone() {
            let mut found = false;
            //if we have a radarr path
            if let Some(radarr_path) = &media_item.radarr_path {
                for link in &jellyfin_links {
                    // if the link in jellyfin contains the radarr path (extension agnostic)
                    if link.contains(
                        &Path::new(&radarr_path)
                            .with_extension("")
                            .to_str()
                            .context(format!(
                                "Could not parse media path for item {:?}",
                                &media_item
                            ))?
                            .to_string(),
                    ) {
                        found = true;
                        let target = std::fs::read_link(Path::new(
                            &[settings.jellyfin_input.to_owned(), link.to_owned()].join(""),
                        ))?;
                        // if the link is still to the radarr file and it has finished transcoding --
                        // replace it
                        if target.starts_with(&settings.radarr_output)
                            && matches!(media_item.status, MediaStatus::Seeding)
                            || matches!(media_item.status, MediaStatus::Completed)
                        {
                            println!("Relinking media item {:?} to Tdarr", &media_item);
                            remove_file(link)?;
                            let tdarr_path = media_item
                                .tdarr_path
                                .clone()
                                .context(format!(
                                    "There is no assigned Tdarr path for item {:?}",
                                    media_item
                                ))?
                                .clone();
                            let original =
                                [settings.tdarr_output.to_owned(), tdarr_path.to_owned()].join("");
                            let link = [settings.jellyfin_input.to_owned(), tdarr_path.to_owned()]
                                .join("/");
                            symlink(original, link)?;
                        }
                    }
                }

                //if the symlink doesn't already exist
                if !found {
                    println!("Link doesn't exist");
                    if matches!(media_item.status, MediaStatus::Seeding)
                        || matches!(media_item.status, MediaStatus::Completed)
                    {
                        // if the media is finished, we create a symlink to the tdarr output
                        let tdarr_path = media_item.tdarr_path.clone().context(format!(
                            "There is no assigned Tdarr path for item {:?}",
                            media_item,
                        ))?;
                        let original =
                            [settings.tdarr_output.to_owned(), tdarr_path.to_owned()].join("");
                        let link =
                            [settings.jellyfin_input.to_owned(), tdarr_path.to_owned()].join("");

                        println!("Linking media item {:?} to Tdarr", &media_item);

                        create_dir_all(Path::new(&link).parent().context(format!(
                            "Could not get parent directory for link target {}",
                            &link
                        ))?)?;

                        symlink(original, link)?;
                    } else if matches!(media_item.status, MediaStatus::Transcoding) {
                        // if the media is still transcoding, we create a symlink to the radarr output
                        let original =
                            [settings.radarr_output.to_owned(), radarr_path.to_owned()].join("");
                        let link =
                            [settings.jellyfin_input.to_owned(), radarr_path.to_owned()].join("");

                        println!("Linking media item {:?} to Tdarr", &media_item);

                        create_dir_all(Path::new(&link).parent().context(format!(
                            "Could not get parent directory for link target {}",
                            &link
                        ))?)?;

                        symlink(original, link)?;
                    }
                }
            }
        }
    }

    // then remove media where it should be removed

    Ok(())
}

struct RadarrMovieResource {
    id: i32,
    //this is a full path as it appears to radarr without prefix -- e.g.
    // /The Dark Knight (2008)/The.Dark.Knight.2008.1080p.BluRay.x264.DTS-HD.MA.5.1-RARBG.mkv
    path: Option<String>,
    title: String,
}

async fn get_radarr_movies(
    radarr_config: &radarr::configuration::Configuration,
) -> Result<Vec<RadarrMovieResource>> {
    let mut res = Vec::new();
    let movies = radarr::movie_api::api_v3_movie_get(radarr_config, None, None, None).await?;
    for movie in movies {
        let movie_path_err_message = format!("No movie path found in resource {:?}", &movie);
        let movie_title_err_message = format!("No title found in resource {:?}", &movie);
        let rel_path_err_message = format!("No relative file path found in resource {:?}", &movie);
        let root_folder_err_message = format!("No root folder path found in resource {:?}", &movie);

        let root_folder_path = movie
            .root_folder_path
            .as_ref()
            .context(root_folder_err_message.clone())?
            .as_ref()
            .context(root_folder_err_message)?;
        res.push(RadarrMovieResource {
            id: movie
                .id
                .context(format!("No movie id found in resource {:?}", &movie))?,
            path: match movie.movie_file {
                Some(r) => Some(
                    [
                        movie
                            .path
                            .as_ref()
                            .context(movie_path_err_message.clone())?
                            .as_ref()
                            .context(movie_path_err_message)?[root_folder_path.len()..]
                            .to_string(),
                        r.relative_path
                            .context(rel_path_err_message.clone())?
                            .context(rel_path_err_message)?,
                    ]
                    .join("/"),
                ),
                None => None,
            },
            title: movie
                .title
                .context(movie_title_err_message.clone())?
                .context(movie_title_err_message)?,
        })
    }
    Ok(res)
}

fn get_file_list_in(parent: &String) -> Result<Vec<String>> {
    let mut res = Vec::new();
    for path in WalkDir::new(parent).into_iter() {
        let path = path?;
        res.push(
            path.path()
                .to_str()
                .context("Could not get path from Tdarr output")?[parent.len()..]
                .to_string(),
        );
    }
    Ok(res)
}

async fn get_tdarr_all_workers(
    tdarr_config: &tdarr::configuration::Configuration,
    tdarr_root_folder: &str,
) -> Result<HashMap<String, f64>> {
    let mut res = HashMap::new();
    //tdarr there are media in the queue, and associated with nodes
    // we have to scan each node -- and then each worker -- and all files
    let tdarr_nodes = tdarr::nodes_api::api_v2_get_nodes_get(tdarr_config)
        .await
        .context("Failed to get Tdarr nodes")?;
    for (_, node) in tdarr_nodes {
        let workers_err_message = "Failed to get Tdarr workers";
        let workers = node
            .as_object()
            .context(workers_err_message)?
            .get("workers")
            .context(workers_err_message)?
            .as_object()
            .context(workers_err_message)?;
        for (_, worker) in workers {
            //TODO:: is there logic here in case a worker isn't processing anything?
            let worker_file_err_message = "Failed to get Tdarr worker file";
            let path_buf = Path::new(
                &worker
                    .get("file")
                    .context(worker_file_err_message)?
                    .as_str()
                    .context(worker_file_err_message)?[tdarr_root_folder.len()..],
            )
            .with_extension("");
            let path = path_buf.to_str().unwrap().to_string();
            let worker_progress_err_message = "Failed to get Tdarr worker progress";
            let progress = worker
                .get("percentage")
                .context(worker_progress_err_message)?
                .as_f64()
                .context(worker_progress_err_message)?;
            res.insert(path, progress);
        }
    }
    Ok(res)
}

struct QbitTorrentProps {
    progress: f64,
    seeding_ratio: f64,
}

async fn get_qbit_torrent_props(
    qbit_config: &mut QbitApi,
    hash: &qbit::torrents::info::TorrentHash,
) -> Result<QbitTorrentProps> {
    let props_value = qbit_config
        .torrents_get_torrent_generic_properties(hash)
        .await
        .map_err(|e| anyhow!("{}", e))?;
    let props = props_value.as_object().context(format!(
        "Could not get torrent properties for item with hash {:?}",
        hash
    ))?;
    let progress_err_message = format!("Could not get progress for item with hash {:?}", hash);
    let progress = props
        .get("progress")
        .context(progress_err_message.clone())?
        .as_f64()
        .context(progress_err_message)?
        * 100.0;
    let seeding_ratio_err_message =
        format!("Could not get seeding ratio for item with hash {:?}", hash);
    let seeding_ratio = props
        .get("share_ratio")
        .context(seeding_ratio_err_message.clone())?
        .as_f64()
        .context(seeding_ratio_err_message)?;
    Ok(QbitTorrentProps {
        progress,
        seeding_ratio,
    })
}

async fn get_qbit_torrent_hashes(
    qbit_config: &mut QbitApi,
) -> Result<HashMap<String, qbit::torrents::info::TorrentHash>> {
    let hashes: HashMap<String, qbit::torrents::info::TorrentHash> = qbit_config
        .torrents_get_hashes()
        .await
        .map_err(|e| anyhow!("{}", e))?
        .into_iter()
        .map(|hash| (hash.hash.clone().to_uppercase(), hash))
        .collect();
    Ok(hashes)
}

async fn get_api_configs(
    settings: &Settings,
    api_configs: Arc<Mutex<ApiConfigs>>,
    state: Arc<Mutex<DaemonState>>,
) -> Result<(
    radarr::configuration::Configuration,
    tdarr::configuration::Configuration,
    QbitApi,
)> {
    let _api_configs = api_configs.lock().unwrap().clone();
    let radarr_config = match _api_configs.radarr_config {
        Some(c) => c,
        None => {
            println!("Attempting connection to Radarr");
            let config = connect_radarr(
                settings.radarr_addr.clone(),
                settings.radarr_api_key.clone(),
                state.clone(),
            )
            .await
            .context("Failed to connect to Radarr")?;
            api_configs.lock().unwrap().radarr_config = Some(config.clone());
            config
        }
    };
    let tdarr_config = match _api_configs.tdarr_config {
        Some(c) => c,
        None => {
            println!("Attempting connection to Tdarr");
            let config = connect_tdarr(settings.tdarr_addr.clone(), state.clone())
                .await
                .context("Failed to connect to Tdarr")?;
            api_configs.lock().unwrap().tdarr_config = Some(config.clone());
            config
        }
    };
    let qbit_config = match _api_configs.qbit_config {
        Some(c) => c,
        None => {
            println!("Attempting connection to qBittorrent");
            let config = connect_qbit(settings.qbit_addr.clone(), state.clone())
                .await
                .context("Failed to connect to qBittorrent")?;
            api_configs.lock().unwrap().qbit_config = Some(config.clone());
            config
        }
    };
    Ok((radarr_config, tdarr_config, qbit_config))
}

async fn get_tdarr_root_folder(
    tdarr_config: &tdarr::configuration::Configuration,
) -> Result<String> {
    let tdarr_root_folder_err_message = "Could not get Tdarr root folder";
    let tdarr_root_folder = tdarr::default_api::api_v2_cruddb_post(
        tdarr_config,
        Some(tdarr_api::models::ApiV2CruddbPostRequest {
            data: Box::new(tdarr_api::models::ApiV2CruddbPostRequestData {
                collection: tdarr_api::models::_api_v2_cruddb_post_request_data::Collection::LibrarySettingsJsondb,
                mode: tdarr_api::models::_api_v2_cruddb_post_request_data::Mode::GetAll,
                doc_id: None,
                obj: None,
            }),
        }),
    )
    .await?.as_array().context(tdarr_root_folder_err_message)?[0].as_object().context(tdarr_root_folder_err_message)?
        .get("folder")
        .context(tdarr_root_folder_err_message)?
        .as_str()
        .context(tdarr_root_folder_err_message)?.to_string();
    Ok(tdarr_root_folder)
}

async fn get_radarr_grabbed_media(
    configuration: &radarr::configuration::Configuration,
) -> Result<HashMap<i32, String>> {
    let grabbed_torrents = radarr::history_api::api_v3_history_since_get(
        configuration,
        None,
        Some(radarr_api::models::MovieHistoryEventType::Grabbed),
        Some(true),
    )
    .await?;
    let mut res = HashMap::new();
    for torrent in grabbed_torrents {
        let download_id_err_message =
            format!("Failed to get download id in resource {:?}", &torrent);
        res.insert(
            torrent
                .movie_id
                .context(format!("No id found in resource {:?}", &torrent))?,
            torrent
                .download_id
                .as_ref()
                .context(download_id_err_message.clone())?
                .as_ref()
                .context(download_id_err_message)?
                .clone(),
        );
    }
    Ok(res)
}

pub async fn connect_radarr(
    radarr_addr: String,
    radarr_api_key: String,
    state: Arc<Mutex<DaemonState>>,
) -> Option<radarr::configuration::Configuration> {
    let mut radarr_config = radarr::configuration::Configuration::new();
    radarr_config.base_path = radarr_addr;
    radarr_config.api_key = Some(radarr::configuration::ApiKey {
        prefix: None,
        key: radarr_api_key,
    });

    let connection_state = match radarr::system_api::api_v3_system_status_get(&radarr_config).await
    {
        Ok(_) => {
            println!("Connected to Radarr successfully");
            ConnectionState::Connected
        }
        Err(radarr::Error::ResponseError(r)) => {
            if r.status == 401 {
                eprintln!("Connection to Radarr successful but unauthorized: {:?}", r);
                ConnectionState::Unauthorized
            } else {
                eprintln!("There was an error connecting to Radarr: {:?}", r);
                ConnectionState::Disconnected
            }
        }
        Err(e) => {
            eprintln!("There was an error while cnnecting to Radarr: {}", e);
            ConnectionState::Disconnected
        }
    };
    let res = match connection_state {
        ConnectionState::Connected => Some(radarr_config),
        _ => None,
    };
    state.lock().unwrap().radarr_connected = connection_state;
    res
}

pub async fn connect_tdarr(
    tdarr_addr: String,
    state: Arc<Mutex<DaemonState>>,
) -> Option<tdarr::configuration::Configuration> {
    let mut tdarr_config = tdarr::configuration::Configuration::new();
    tdarr_config.base_path = tdarr_addr;

    let connection_state = match tdarr::default_api::api_v2_status_get(&tdarr_config).await {
        Ok(_) => {
            println!("Connected to Tdarr successfully");
            ConnectionState::Connected
        }
        Err(e) => {
            eprintln!("There was an error connecting to Tdarr: {:?}", e);
            ConnectionState::Disconnected
        }
    };
    let res = match connection_state {
        ConnectionState::Connected => Some(tdarr_config),
        _ => None,
    };
    state.lock().unwrap().tdarr_connected = connection_state;
    res
}

pub async fn connect_qbit(qbit_addr: String, state: Arc<Mutex<DaemonState>>) -> Option<QbitApi> {
    let qbit_api = QbitApi::new(
        qbit_addr,
        qbittorrent_rust::core::creds::Credentials::new("", ""),
    )
    .await;
    let mut res = None;
    let connection_state = match qbit_api {
        Ok(q) => {
            println!("Connected to qBittorrent successfully");
            res = Some(q);
            ConnectionState::Connected
        }
        Err(e) => match e.err_type {
            qbittorrent_rust::error_handling::error_type::ErrorType::WrongCreds => {
                eprintln!(
                    "Connection to qBittorrent successful but unauthorized, please whitelist Pidarr in the qBittorrent client: {:?}",
                    e
                );
                ConnectionState::Unauthorized
            }
            _ => {
                eprintln!("There was an error connecting to qBittorrent: {:?}", e);
                ConnectionState::Disconnected
            }
        },
    };
    state.lock().unwrap().qbit_connected = connection_state.clone();
    res
}
