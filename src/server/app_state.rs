use std::sync::Arc;
use crate::server::config::ServerConfig;

pub type SharedState = Arc<AppState>;
pub struct AppState{
    pub name:String,
    pub server_config: ServerConfig
}

impl AppState{
    pub fn new(server_config: ServerConfig) -> Self{
        Self{
            name: "qt-tool".to_string(),
            server_config,
        }
    }
}