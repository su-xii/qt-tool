use crate::cli::Cli;
use crate::handler::Handler;
use crate::server::app_state::AppState;
use crate::server::config::{OutputItem, ServerConfig};
use crate::server::router::user::UserRouter;
use crate::server::router::{combine_routers, ProcessRouter};
use axum::Router;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub struct ServerHandler;

impl Handler for ServerHandler {
    async fn run(cli: &Cli) -> anyhow::Result<()> {
        println!("以服务器模式运行了");

        // 加载配置
        let config = ServerConfig::load_form_file(cli.config())?;
        config.validate().map_err(|e|anyhow::anyhow!(e))?;

        // 校验配置目录
        for output_item in &config.output{
            check_target_dir(output_item).map_err(|e|anyhow::anyhow!(e))?;
        }

        // 全局状态
        let state = Arc::new(AppState::new(config));
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0,),9999)).await?;

        // 组合路由
        let app = combine_routers(Router::new(),vec![
           Box::new(UserRouter),
           Box::new(ProcessRouter)
        ]).with_state(state).layer(CorsLayer::permissive());

        // 启动服务
        axum::serve(listener,app).await.map_err(|e|anyhow::anyhow!(e))
    }
}

// 校验目标目录是否存在
fn check_target_dir(output_config:&OutputItem) -> anyhow::Result<(), String> {
    let path = Path::new(&output_config.base_path);
    if !path.exists() {
        return Err(format!("配置（{}）不存在目录：{}", &output_config.name,&output_config.base_path).into());
    }
    Ok(())
}


#[test]
fn test_check(){
    let item = OutputItem{
        name: "测试名字".to_string(),
        description: "描述哈哈哈".to_string(),
        base_path: "test".to_string(),
        format: vec![String::from("vv1"),String::from("vv2"),String::from("vv3")],
        zip_format: vec![String::from("."),String::from("v1"),String::from("v2")],
    };
    check_target_dir(&item).unwrap();
}
