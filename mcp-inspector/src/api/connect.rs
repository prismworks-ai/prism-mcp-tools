use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    api::AppState,
    inspector::InspectorClient,
    models::ConnectionInfo,
};

#[derive(Debug, Deserialize)]
pub struct ConnectRequest {
    pub url: String,
    pub transport: String, // "http", "websocket", "stdio"
    pub headers: Option<Vec<(String, String)>>,
}

#[derive(Debug, Serialize)]
pub struct ConnectResponse {
    pub success: bool,
    pub message: String,
    pub connection_info: Option<ConnectionInfo>,
}

pub async fn connect(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ConnectRequest>,
) -> Result<Json<ConnectResponse>, StatusCode> {
    // Create new inspector client
    let client = match InspectorClient::connect(&request.url, &request.transport).await {
        Ok(client) => client,
        Err(e) => {
            return Ok(Json(ConnectResponse {
                success: false,
                message: format!("Failed to connect: {}", e),
                connection_info: None,
            }));
        }
    };

    // Get server info
    let server_info = client.get_server_info().await.ok();

    let connection_info = ConnectionInfo {
        url: request.url.clone(),
        transport: request.transport.clone(),
        connected: true,
        server_name: server_info.as_ref().map(|s| s.name.clone()),
        server_version: server_info.as_ref().map(|s| s.version.clone()),
    };

    // Store client and connection info
    *state.client.write().await = Some(Arc::new(client));
    *state.connection_info.write().await = Some(connection_info.clone());

    Ok(Json(ConnectResponse {
        success: true,
        message: "Connected successfully".to_string(),
        connection_info: Some(connection_info),
    }))
}

pub async fn disconnect(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ConnectResponse>, StatusCode> {
    // Clear client and connection info
    *state.client.write().await = None;
    *state.connection_info.write().await = None;

    Ok(Json(ConnectResponse {
        success: true,
        message: "Disconnected".to_string(),
        connection_info: None,
    }))
}

pub async fn status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ConnectionInfo>, StatusCode> {
    match state.connection_info.read().await.as_ref() {
        Some(info) => Ok(Json(info.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}