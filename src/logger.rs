use std::fs::OpenOptions;
use anyhow::Result;
use tracing::Metadata;
use tracing_subscriber::{
    prelude::*,
    Registry,
    fmt,
};
use tracing_subscriber::layer::Context;
use crate::record::logger_config_path;

pub fn init() -> Result<()>{
    let mut layers: Vec<Box<dyn tracing_subscriber::Layer<_> + Send + Sync>> = Vec::new();

    let log_path = logger_config_path()?;
    let file_writer = move || {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("打开日志文件失败")
    };

    let time_format = time::format_description::parse("[hour]:[minute]:[second]")
        .expect("format string should be valid!");

    let net_layer = fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_filter(NetFilter);
    layers.push(Box::new(net_layer));

    let general_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true) // 启用颜色
        .with_timer(fmt::time::LocalTime::new(time_format))
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_level(true)
        .with_filter(GeneralFilter);
    layers.push(Box::new(general_layer));

    // 初始化日志
    let subscriber = Registry::default().with(layers);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

struct NetFilter;
struct GeneralFilter;

impl <S> tracing_subscriber::layer::Filter<S> for NetFilter{
    /// 判断某个事件是否应该被记录
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        app_filter(meta) && net_filter(meta)
    }
}

impl <S> tracing_subscriber::layer::Filter<S> for GeneralFilter{
    /// 判断某个事件是否应该被记录
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        app_filter(meta) && !net_filter(meta)
    }
}


fn app_filter(meta: &Metadata<'_>) -> bool{
    meta.target().contains("qt_tool")
}

fn net_filter(meta: &Metadata<'_>) -> bool{
    let target = meta.target();
    target.contains("qt_tool::net")||target.contains("qt_tool::record")
}