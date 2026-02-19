mod handler;
pub use handler::CliHandler;

use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;
use crate::handler::Handler;
use crate::server::ServerHandler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

    /// 输入的 ZIP 文件路径（仅命令行模式使用）
    zip_path: Option<String>,

    /// 输出的基础文件名（如 my_icon.png），可选（仅命令行模式使用）
    #[arg(value_parser = parse_output_name)]
    output_name: Option<String>,

    /// 启动 HTTP 服务器模式
    #[arg(long, default_value_t = false)]
    run: bool,

    /// 启动 HTTP 服务器模式的配置文件路径
    #[arg(long="config",short = 'c', default_value_t = String::from("config.toml"))]
    config: String,

    /// 指定输出目录（默认为当前目录）
    #[arg(short = 'p', long = "output-dir", value_name = "DIR")]
    output_dir: Option<PathBuf>,
}


fn parse_output_name(s: &str) -> Result<String, String> {
    if !s.ends_with(".png") {
        Err("输出文件名必须以 .png 结尾".to_string())
    } else {
        Ok(s.to_string())
    }
}

impl Cli{
    pub fn is_server_mode(&self) -> bool{
        self.run
    }

    pub fn zip_path(&self) -> Option<String>{
        self.zip_path.clone()
    }

    pub fn output_name(&self) -> Option<String>{
        self.output_name.clone()
    }

    pub fn output_dir(&self) -> Option<PathBuf>{
        self.output_dir.clone()
    }

    pub fn config(&self) -> String{
        self.config.clone()
    }

    pub async fn run(&self) -> Result<()>{
        if self.is_server_mode(){
            ServerHandler::run(self).await
        } else {
            CliHandler::run(self).await
        }
    }

}
