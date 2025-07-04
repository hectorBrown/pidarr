/*
 * Tdarr API
 *
 * Tdarr API Docs
 *
 * The version of the OpenAPI document: 2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`api_v2_alter_worker_limit_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2AlterWorkerLimitPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_cancel_worker_item_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2CancelWorkerItemPostError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_disconnect_node_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2DisconnectNodePostError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_download_plugins_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2DownloadPluginsGetError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_file_download_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2FileDownloadPostError {
    Status400(String),
    Status403(String),
    Status501(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_file_upload_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2FileUploadPostError {
    Status400(String),
    Status403(String),
    Status500(String),
    Status501(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_get_new_task_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2GetNewTaskPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_get_node_log_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2GetNodeLogPostError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_get_nodes_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2GetNodesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_item_proc_end_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2ItemProcEndPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_kill_worker_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2KillWorkerPostError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_log_job_report_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2LogJobReportPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_poll_worker_limits_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2PollWorkerLimitsPostError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_read_plugin_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2ReadPluginPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_restart_node_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2RestartNodePostError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_sync_plugins_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2SyncPluginsPostError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_update_node_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2UpdateNodePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v2_update_node_relay_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV2UpdateNodeRelayPostError {
    UnknownValue(serde_json::Value),
}


/// For changing the number of running workers of a specific type on a specific node
pub async fn api_v2_alter_worker_limit_post(configuration: &configuration::Configuration, body: Option<models::ApiV2AlterWorkerLimitPostRequest>) -> Result<String, Error<ApiV2AlterWorkerLimitPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/alter-worker-limit", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2AlterWorkerLimitPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For cancelling a running worker item on a specific node
pub async fn api_v2_cancel_worker_item_post(configuration: &configuration::Configuration, body: Option<models::ApiV2CancelWorkerItemPostRequest>) -> Result<String, Error<ApiV2CancelWorkerItemPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/cancel-worker-item", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2CancelWorkerItemPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For forcefully disconnecting a node
pub async fn api_v2_disconnect_node_post(configuration: &configuration::Configuration, body: Option<models::ApiV2DisconnectNodePostRequest>) -> Result<String, Error<ApiV2DisconnectNodePostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/disconnect-node", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2DisconnectNodePostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For nodes to download the latest plugins zip
pub async fn api_v2_download_plugins_get(configuration: &configuration::Configuration, ) -> Result<String, Error<ApiV2DownloadPluginsGetError>> {

    let uri_str = format!("{}/api/v2/download-plugins", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2DownloadPluginsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For downloading a file
pub async fn api_v2_file_download_post(configuration: &configuration::Configuration, body: Option<models::ApiV2FileDownloadPostRequest>) -> Result<serde_json::Value, Error<ApiV2FileDownloadPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/file/download", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2FileDownloadPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For uploading a file
pub async fn api_v2_file_upload_post(configuration: &configuration::Configuration, ) -> Result<serde_json::Value, Error<ApiV2FileUploadPostError>> {

    let uri_str = format!("{}/api/v2/file/upload", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2FileUploadPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For a node to request a new task
pub async fn api_v2_get_new_task_post(configuration: &configuration::Configuration, body: Option<models::ApiV2GetNewTaskPostRequest>) -> Result<Vec<serde_json::Value>, Error<ApiV2GetNewTaskPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/get-new-task", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;serde_json::Value&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;serde_json::Value&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2GetNewTaskPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For getting the log of a node
pub async fn api_v2_get_node_log_post(configuration: &configuration::Configuration, body: Option<models::ApiV2DisconnectNodePostRequest>) -> Result<String, Error<ApiV2GetNodeLogPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/get-node-log", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2GetNodeLogPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For getting connected nodes information
pub async fn api_v2_get_nodes_get(configuration: &configuration::Configuration, ) -> Result<std::collections::HashMap<String, serde_json::Value>, Error<ApiV2GetNodesGetError>> {

    let uri_str = format!("{}/api/v2/get-nodes", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `std::collections::HashMap&lt;String, serde_json::Value&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `std::collections::HashMap&lt;String, serde_json::Value&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2GetNodesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For when a node completes processing an item
pub async fn api_v2_item_proc_end_post(configuration: &configuration::Configuration, body: Option<models::ApiV2ItemProcEndPostRequest>) -> Result<String, Error<ApiV2ItemProcEndPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/item-proc-end", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2ItemProcEndPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For killing a worker on a node
pub async fn api_v2_kill_worker_post(configuration: &configuration::Configuration, body: Option<models::ApiV2KillWorkerPostRequest>) -> Result<String, Error<ApiV2KillWorkerPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/kill-worker", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2KillWorkerPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For updating a job report
pub async fn api_v2_log_job_report_post(configuration: &configuration::Configuration, body: Option<models::ApiV2LogJobReportPostRequest>) -> Result<String, Error<ApiV2LogJobReportPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/log-job-report", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2LogJobReportPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For a node to get its worker limits and check if there's anything in the queue
pub async fn api_v2_poll_worker_limits_post(configuration: &configuration::Configuration, body: Option<models::ApiV2DisconnectNodePostRequest>) -> Result<models::ApiV2PollWorkerLimitsPost200Response, Error<ApiV2PollWorkerLimitsPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/poll-worker-limits", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiV2PollWorkerLimitsPost200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiV2PollWorkerLimitsPost200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2PollWorkerLimitsPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For a node to read a plugin
pub async fn api_v2_read_plugin_post(configuration: &configuration::Configuration, body: Option<models::ApiV2ReadPluginPostRequest>) -> Result<models::ApiV2ReadPluginPost200Response, Error<ApiV2ReadPluginPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/read-plugin", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiV2ReadPluginPost200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiV2ReadPluginPost200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2ReadPluginPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For restarting a specific node
pub async fn api_v2_restart_node_post(configuration: &configuration::Configuration, body: Option<models::ApiV2RestartNodePostRequest>) -> Result<String, Error<ApiV2RestartNodePostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/restart-node", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2RestartNodePostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For syncing plugins from server to all nodes
pub async fn api_v2_sync_plugins_post(configuration: &configuration::Configuration, ) -> Result<String, Error<ApiV2SyncPluginsPostError>> {

    let uri_str = format!("{}/api/v2/sync-plugins", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2SyncPluginsPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For the UI to update a connected node
pub async fn api_v2_update_node_post(configuration: &configuration::Configuration, body: Option<models::ApiV2UpdateNodePostRequest>) -> Result<String, Error<ApiV2UpdateNodePostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/update-node", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2UpdateNodePostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For nodes to update the server with their status
pub async fn api_v2_update_node_relay_post(configuration: &configuration::Configuration, body: Option<models::ApiV2UpdateNodeRelayPostRequest>) -> Result<String, Error<ApiV2UpdateNodeRelayPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!("{}/api/v2/update-node-relay", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `String`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApiV2UpdateNodeRelayPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

