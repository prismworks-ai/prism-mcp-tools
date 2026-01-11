use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::api::AppState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    ConnectionStatus {
        connected: bool,
        url: Option<String>,
    },
    ToolResponse {
        tool: String,
        result: serde_json::Value,
        duration_ms: u64,
    },
    MetricsUpdate {
        requests_per_second: f64,
        average_latency_ms: f64,
        error_rate: f64,
    },
    Error {
        message: String,
    },
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    info!("WebSocket connection established");

    // Send initial connection status
    let connection_info = state.connection_info.read().await;
    let status_msg = WsMessage::ConnectionStatus {
        connected: connection_info.is_some(),
        url: connection_info.as_ref().map(|info| info.url.clone()),
    };
    
    if let Ok(msg) = serde_json::to_string(&status_msg) {
        let _ = socket.send(axum::extract::ws::Message::Text(msg)).await;
    }

    // Handle incoming messages
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                axum::extract::ws::Message::Text(text) => {
                    // Handle text messages from client
                    info!("Received WebSocket message: {}", text);
                }
                axum::extract::ws::Message::Close(_) => {
                    info!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}