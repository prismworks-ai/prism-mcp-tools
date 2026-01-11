use anyhow::Result;
use axum::Router;
use clap::Parser;
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod inspector;
mod models;
mod server;
mod websocket;

use server::create_app;

/// MCP Inspector - Web-based tool for testing and debugging MCP servers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to run the inspector on
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Host to bind to
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    /// Auto-connect to server on startup
    #[arg(short, long)]
    connect: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Open browser automatically
    #[arg(short = 'o', long)]
    open: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    let log_level = if args.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("mcp_inspector={},tower_http=info", log_level).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting MCP Inspector v{}", env!("CARGO_PKG_VERSION"));

    // Create the application
    let app = create_app().await?;

    // Parse address
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;

    info!("🚀 MCP Inspector running at http://{}", addr);

    // Open browser if requested
    if args.open {
        if let Err(e) = open::that(format!("http://{}", addr)) {
            tracing::warn!("Failed to open browser: {}", e);
        }
    }

    // Print startup information
    println!("\n🔍 MCP Inspector");
    println!("==========================================\n");
    println!("  Web UI:   http://{}", addr);
    println!("  API:      http://{}/api", addr);
    println!("  Docs:     http://{}/api/docs\n", addr);
    println!("Press Ctrl+C to stop\n");

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}