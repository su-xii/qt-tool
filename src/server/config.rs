use std::fs;
use std::path::Path;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug,Deserialize,Clone)]
pub struct ServerConfig{
    pub server: Server,
    pub output: Vec<OutputItem>
}

#[derive(Deserialize, Debug,Clone)]
pub struct Server{
    pub addr:String
}

#[derive(Deserialize, Debug,Clone)]
pub struct OutputItem {
    pub name: String,
    pub description: String,
    #[serde(rename = "base_path")]
    pub base_path: String,
    pub format: Vec<String>,
    pub zip_format: Vec<String>,
    pub format_limit: Vec<String>
}

impl ServerConfig{
    pub fn load_form_file<P: AsRef<Path>>(path: P) -> Result<Self>{
        let contents = fs::read_to_string(path)?;
        let config: ServerConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        for (i, item) in self.output.iter().enumerate() {
            if item.name.is_empty() {
                return Err(format!("配置项 {} 的名称不能为空", i + 1));
            }
            if item.base_path.is_empty() {
                return Err(format!("配置项 {} 的路径不能为空", i + 1));
            }
            if item.format.is_empty() {
                return Err(format!("配置项 {} 的格式不能为空", i + 1));
            }
            if item.zip_format.is_empty() {
                return Err(format!("配置项 {} 的压缩格式不能为空", i + 1));
            }
            if item.format.len() != item.zip_format.len() {
                return Err(format!("配置项 {} 的格式和压缩格式长度不一致", i + 1));
            }
            if item.format_limit.len() <= 0{
                return Err(format!("配置项 {} 的格式限制必须大于0", i + 1));
            }
        }
        Ok(())
    }

}