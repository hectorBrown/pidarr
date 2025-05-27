use anyhow::{Context, Result, anyhow};
use pidarr_shared::{ConnectionState, DaemonState, Media, MediaStatus, Settings};
use qbittorrent_rust::{api_fns as qbit, core::api::QbitApi};
use radarr_api::apis as radarr;
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
    let radarr_root_folder = get_radarr_root_folder(&radarr_config).await?;
    let tdarr_root_folder = get_tdarr_root_folder(&tdarr_config).await?;

    //
    //RADARR SECTION
    //

    //loop through all the grabbed torrents in radarr's history
    let radarr_grabbed_media =
        get_radarr_grabbed_media(&radarr_config, &radarr_root_folder).await?;
    for item in radarr_grabbed_media {
        // if the daemon state does not have an entry for this id, we add it
        let mut media = state.lock().unwrap().media.clone();
        if let std::collections::hash_map::Entry::Vacant(_) = media.entry(item.path.clone()) {
            //we insert the media item here, unfinished
            let media = Media {
                title: item.title.clone(),
                download_id: item.download_id,
                download_progress: None,
                seeding_ratio: None,
                transcode_progress: None,
                status: MediaStatus::Unknown,
            };
            println!(
                "-----------\nFound movie: {} at path {}\n{:?}\n-----------",
                &item.title, &item.path, &media
            );
            state.lock().unwrap().media.insert(item.path, media);
        }
    }

    //
    //QBIT SECTION
    //

    //grab all hashes that are in qbittorrent
    let hashes = get_qbit_torrent_hashes(&mut qbit_config).await?;
    // for each item of media
    let media = state.lock().unwrap().media.clone();
    for (id, item) in media {
        // match radarr's download_id with the hashes in qBittorrent
        let hash = hashes
            .get(&item.download_id)
            .context(format!("Could not find torrent hash for item {:?}", &item))?;
        let props = get_qbit_torrent_props(&mut qbit_config, hash).await?;
        let progress = props.progress;
        let mut state = state.lock().unwrap();
        let media = state
            .media
            .get_mut(&id)
            .context(format!("Could not get media object for item {:?}", &item))?;
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

    //
    // TDARR SECTION
    //

    //tdarr there are media in the queue, and associated with nodes
    // we have to scan each node -- and then each worker -- and all files
    let workers = get_tdarr_all_workers(&tdarr_config, &tdarr_root_folder).await?;
    for worker in workers {
        let mut state = state.lock().unwrap();
        let media = state.media.get_mut(&worker.path).context(format!(
            "Could not get media object for item {:?}",
            &worker.path
        ))?;
        //update transcode progress for each torrent
        media.transcode_progress = Some(worker.progress);
    }

    let tdarr_output_list = get_tdarr_output_list(settings.tdarr_output)?;
    let _state = state.lock().unwrap().clone();
    let media_paths = _state.media.keys();
    for media_path in media_paths {
        // if the media item is in the tdarr output list, we update its status
        for path in &tdarr_output_list {
            if path.contains(media_path) {
                //media is in tdarr output
                let mut state = state.lock().unwrap();
                let media = state.media.get_mut(media_path).context(format!(
                    "Could not get media object for item {:?}",
                    media_path
                ))?;
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

    // now act based on status to create and remove softlinks
    //

    // then remove media where it should be removed

    Ok(())
}

struct TdarrWorker {
    path: String,
    progress: f64,
}

fn get_tdarr_output_list(tdarr_output: String) -> Result<Vec<String>> {
    let mut res = Vec::new();
    for path in WalkDir::new(tdarr_output).into_iter() {
        let path = path?;
        res.push(
            path.path()
                .to_str()
                .context("Could not get path from Tdarr output")?
                .to_string(),
        );
    }
    Ok(res)
}

async fn get_tdarr_all_workers(
    tdarr_config: &tdarr::configuration::Configuration,
    tdarr_root_folder: &str,
) -> Result<Vec<TdarrWorker>> {
    let mut res = Vec::new();
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
            let path = path_buf.to_str().unwrap();
            // TODO: this is so fucked -- i have no idea why but the radarr api outputs a filename
            // that is one character shorter -- if any torrents differ by only 1 character, i guess
            // you bring it on yourself... still makes me deeply unhappy
            let path = path[..path.len() - 1].to_string();
            let worker_progress_err_message = "Failed to get Tdarr worker progress";
            let progress = worker
                .get("percentage")
                .context(worker_progress_err_message)?
                .as_f64()
                .context(worker_progress_err_message)?;
            res.push(TdarrWorker { path, progress });
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

async fn get_radarr_root_folder(
    radarr_config: &radarr::configuration::Configuration,
) -> Result<String> {
    let radarr_root_folder_err_message = "Could not get root folder path from Radarr";
    let radarr_root_folder = radarr::root_folder_api::api_v3_rootfolder_get(radarr_config).await?
        [0]
    .path
    .clone()
    .context(radarr_root_folder_err_message)?
    .context(radarr_root_folder_err_message)?;
    Ok(radarr_root_folder)
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

struct GrabbedMediaResource {
    path: String,
    download_id: String,
    title: String,
}

async fn get_radarr_grabbed_media(
    configuration: &radarr::configuration::Configuration,
    radarr_root_folder: &str,
) -> Result<Vec<GrabbedMediaResource>> {
    let grabbed_torrents = radarr::history_api::api_v3_history_since_get(
        configuration,
        None,
        Some(radarr_api::models::MovieHistoryEventType::Grabbed),
        Some(true),
    )
    .await?;
    let mut res = Vec::new();
    for torrent in grabbed_torrents {
        let movie = torrent
            .movie
            .as_ref()
            .context(format!("No movie found in resource {:?}", &torrent))?;
        let movie_path_err_message = format!("No movie path found in resource {:?}", &torrent);
        let parent_path = movie
            .path
            .as_ref()
            .context(movie_path_err_message.clone())?
            .as_ref()
            .context(movie_path_err_message)?[radarr_root_folder.len()..]
            .to_string();
        let filename_err_message = format!("No source title found in resource {:?}", &torrent);
        let filename = torrent
            .source_title
            .as_ref()
            .context(filename_err_message.clone())?
            .as_ref()
            .context(filename_err_message)?;
        let path = [parent_path, filename.to_string()].join("/");
        let title_err_message = format!("No title found in resource {:?}", &torrent);
        let title = movie
            .title
            .as_ref()
            .context(title_err_message.clone())?
            .as_ref()
            .context(title_err_message)?
            .clone();
        let download_id_err_message =
            format!("Failed to get download id in resource {:?}", &torrent);
        let download_id = torrent
            .download_id
            .as_ref()
            .context(download_id_err_message.clone())?
            .as_ref()
            .context(download_id_err_message)?
            .clone();
        res.push(GrabbedMediaResource {
            path,
            download_id,
            title,
        });
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
