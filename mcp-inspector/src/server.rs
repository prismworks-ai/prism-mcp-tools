use anyhow::Result;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};

use crate::{
    api::{self, AppState},
    websocket,
};

pub async fn create_app() -> Result<Router> {
    // Create shared application state
    let state = Arc::new(AppState::new());

    // API routes
    let api_routes = Router::new()
        .route("/connect", post(api::connect))
        .route("/disconnect", post(api::disconnect))
        .route("/status", get(api::status))
        .route("/tools", get(api::list_tools))
        .route("/tools/:name", get(api::get_tool))
        .route("/tools/:name/invoke", post(api::invoke_tool))
        .route("/sessions", get(api::list_sessions).post(api::save_session))
        .route("/sessions/:id", get(api::get_session).delete(api::delete_session));

    // Main application router
    let app = Router::new()
        // Serve the main page
        .route("/", get(index_handler))
        // API routes
        .nest("/api", api_routes)
        // WebSocket endpoint
        .route("/ws", get(websocket::websocket_handler))
        // Serve static files
        .nest_service("/static", ServeDir::new("static"))
        // Add state
        .with_state(state)
        // Add middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    Ok(app)
}

async fn index_handler() -> Html<String> {
    // Read the index.html file
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(html) => Html(html),
        Err(_) => {
            // Fallback HTML if file not found
            Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MCP Inspector</title>
    <link rel="stylesheet" href="/static/css/inspector.css">
</head>
<body>
    <div id="app">
        <h1>🔍 MCP Inspector</h1>
        <p>Error: Could not load index.html. Please ensure the static files are in place.</p>
    </div>
    <script src="/static/js/inspector.js"></script>
</body>
</html>"#.to_string())
        }
    }
}