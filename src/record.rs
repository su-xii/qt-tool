use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use crate::net::upload_record_auto;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Record {
    pub key: String,
    pub is_success: bool,
    pub timestamp: i64,
}

impl Record {
    pub fn new(key: String, is_success: bool, timestamp: i64) -> Self {
        Record {
            key,
            is_success,
            timestamp,
        }
    }
}

// 加载记录
pub fn load_records() -> Result<Vec<Record>> {
    let path = config_path()?;

    // 文件不存在时返回空
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(&path).with_context(|| format!("无法打开配置文件：{:?}", path))?;
    let reader = BufReader::new(file);
    let records: Vec<Record> = serde_json::from_reader(reader).with_context(|| "解析配置文件失败")?;
    Ok(records)
}

// 保存记录（原子写入）
pub fn save_records(records: &Vec<Record>) -> Result<()> {
    let dir = config_dir()?;
    let path = config_path()?;

    // 确保目录存在
    fs::create_dir_all(&dir)
        .with_context(|| format!("无法创建配置目录：{:?}", dir))?;

    // 原子写入：先写临时文件，再重命名
    let temp_path = dir.join("config.json.tmp");
    let file = File::create(&temp_path)
        .with_context(|| format!("无法创建临时文件：{:?}", temp_path))?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, records)
        .with_context(|| "序列化配置失败")?;

    // 重命名临时文件为正式文件（原子操作）
    fs::rename(&temp_path, &path)
        .with_context(|| format!("无法保存配置文件：{:?}", path))?;

    Ok(())
}

pub fn add_record(record: Record) -> Result<()> {
    let mut records = load_records()?;
    records.push(record);
    save_records(&records)?;
    Ok(())
}


fn config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("获取配置目录失败"))
        .map(|p| p.join("qt-tool"))
}

fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("record.json"))
}

pub fn logger_config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("record.log"))
}

pub async fn init(){
    if let Err(e) = upload_record_auto().await{
        tracing::error!("上传日志失败: {:?}", e);
    }
}
