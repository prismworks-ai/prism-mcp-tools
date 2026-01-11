use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::api::AppState;

#[derive(Debug, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Value,
}

#[derive(Debug, Deserialize)]
pub struct InvokeRequest {
    pub arguments: Value,
}

#[derive(Debug, Serialize)]
pub struct InvokeResponse {
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
    pub duration_ms: u64,
}

pub async fn list_tools(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Tool>>, StatusCode> {
    let client = state.client.read().await;
    let client = client.as_ref().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    match client.list_tools().await {
        Ok(tools) => Ok(Json(
            tools
                .into_iter()
                .map(|t| Tool {
                    name: t.name,
                    description: t.description,
                    input_schema: t.input_schema,
                })
                .collect(),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_tool(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Tool>, StatusCode> {
    let client = state.client.read().await;
    let client = client.as_ref().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    match client.get_tool(&name).await {
        Ok(tool) => Ok(Json(Tool {
            name: tool.name,
            description: tool.description,
            input_schema: tool.input_schema,
        })),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn invoke_tool(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Json(request): Json<InvokeRequest>,
) -> Result<Json<InvokeResponse>, StatusCode> {
    let client = state.client.read().await;
    let client = client.as_ref().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let start = std::time::Instant::now();
    
    match client.invoke_tool(&name, request.arguments).await {
        Ok(result) => {
            let duration_ms = start.elapsed().as_millis() as u64;
            Ok(Json(InvokeResponse {
                success: true,
                result: Some(result),
                error: None,
                duration_ms,
            }))
        }
        Err(e) => {
            let duration_ms = start.elapsed().as_millis() as u64;
            Ok(Json(InvokeResponse {
                success: false,
                result: None,
                error: Some(e.to_string()),
                duration_ms,
            }))
        }
    }
}