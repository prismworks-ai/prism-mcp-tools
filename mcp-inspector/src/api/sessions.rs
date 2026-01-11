use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{api::AppState, models::Session};

#[derive(Debug, Deserialize)]
pub struct SaveSessionRequest {
    pub name: String,
    pub description: Option<String>,
}

pub async fn list_sessions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Session>>, StatusCode> {
    Ok(Json(state.sessions.read().await.clone()))
}

pub async fn save_session(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SaveSessionRequest>,
) -> Result<Json<Session>, StatusCode> {
    let connection_info = state
        .connection_info
        .read()
        .await
        .clone()
        .ok_or(StatusCode::PRECONDITION_FAILED)?;

    let session = Session {
        id: Uuid::new_v4(),
        name: request.name,
        description: request.description,
        connection_info,
        created_at: chrono::Utc::now(),
        requests: Vec::new(), // TODO: Store request history
    };

    state.sessions.write().await.push(session.clone());

    Ok(Json(session))
}

pub async fn get_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Session>, StatusCode> {
    state
        .sessions
        .read()
        .await
        .iter()
        .find(|s| s.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn delete_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut sessions = state.sessions.write().await;
    let len_before = sessions.len();
    sessions.retain(|s| s.id != id);
    
    if sessions.len() < len_before {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}