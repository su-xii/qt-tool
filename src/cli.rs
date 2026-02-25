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

    /// 输出的文件目录列表。
    /// 格式: [v1, v2, v3] 或 [v1 v2 v3]
    /// 示例: -o "[small, medium, large]"
    #[arg(short = 'o', long = "outputs",default_value_t = String::from(""))]
    pub output_dirs_raw: String
}


fn parse_output_name(s: &str) -> Result<String, String> {
    if !s.ends_with(".png") {
        Err("输出文件名必须以 .png 结尾".to_string())
    } else {
        Ok(s.to_string())
    }
}

/// 自定义解析器：将 "[a, b, c]" 或 "[a b c]" 解析为 Vec<String>
fn parse_string_array(s: &str) -> Result<Vec<String>, String> {
    let s = s.trim();

    // 检查是否以 [ 开头和 ] 结尾
    if !s.starts_with('[') || !s.ends_with(']') {
        return Err("格式错误：参数必须以 [ 开头并以 ] 结尾。\n例如: -o \"[v1, v2, v3]\"".to_string());
    }

    // 去掉方括号
    let content = &s[1..s.len() - 1];

    // 如果内容为空，返回空数组
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    // 分割字符串：支持逗号分隔 或 空格分隔
    let items: Vec<String> = if content.contains(',') {
        content.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        content.split_whitespace()
            .map(|s| s.to_string())
            .collect()
    };

    // 清理每个元素周围的引号
    let cleaned_items = items.into_iter().map(|item| {
        let trimmed = item.trim();
        if (trimmed.starts_with('"') && trimmed.ends_with('"')) ||
            (trimmed.starts_with('\'') && trimmed.ends_with('\'')) {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            trimmed.to_string()
        }
    }).collect();

    Ok(cleaned_items)
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

    pub fn output_dirs(&self) -> Vec<String>{
        parse_string_array(&self.output_dirs_raw).expect("output_dirs参数错误")
    }

    pub async fn run(&self) -> Result<()>{
        if self.is_server_mode(){
            ServerHandler::run(self).await
        } else {
            CliHandler::run(self).await
        }
    }

}
