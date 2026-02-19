use serde::{Deserialize, Serialize};
use crate::server::config::ServerConfig;

/// 处理程序请求
#[derive(Deserialize)]
pub struct ProcessRequest {
    pub name: String, // e.g., "icon.png"
    pub path: String, // e.g., "assets.zip"
    pub config_index: usize, // e.g., 0
}

/// 校验文件请求
#[derive(Deserialize)]
pub struct CheckRequest {
    pub name: String,
    pub config_index: usize,
}

// 查询参数结构体
#[derive(Deserialize)]
pub struct LimitsQuery {
    pub config_index: usize,
}


/// 获取可选择的配置响应
#[derive(Serialize)]
pub struct ConfigsResponse{
    name: String,
    description: String,
    base_path: String,
}

impl ConfigsResponse{
    pub fn from_server_config(config: &ServerConfig) -> Vec<Self> {
        config.output.iter().map(|e|Self{
            name: e.name.clone(),
            description: e.description.clone(),
            base_path: e.base_path.clone(),
        }).collect::<Vec<_>>()
    }
}