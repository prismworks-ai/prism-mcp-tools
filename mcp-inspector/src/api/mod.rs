use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    inspector::InspectorClient,
    models::{ConnectionInfo, Session},
};

pub mod connect;
pub mod sessions;
pub mod tools;

pub use connect::*;
pub use sessions::*;
pub use tools::*;

/// Shared application state
pub struct AppState {
    /// Current MCP client connection
    pub client: RwLock<Option<Arc<InspectorClient>>>,
    
    /// Saved sessions
    pub sessions: RwLock<Vec<Session>>,
    
    /// Current connection info
    pub connection_info: RwLock<Option<ConnectionInfo>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: RwLock::new(None),
            sessions: RwLock::new(Vec::new()),
            connection_info: RwLock::new(None),
        }
    }
}