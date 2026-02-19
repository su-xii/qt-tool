mod dto;

use std::path::Path;
use axum::extract::{Query, State};
use dto::ConfigsResponse;
use crate::server::app_state::{SharedState};
use crate::server::router::AppRouter;
use crate::server::util::result_util::ResultUtil;
use axum::routing::{get, post};
use axum::{Json, Router};
use crate::handler::{check_file_exists, handle_unzip_file};
use crate::server::router::process::dto::{CheckRequest, LimitsQuery, ProcessRequest};

pub struct ProcessRouter;

impl AppRouter<SharedState> for ProcessRouter {
    fn create_router(&self, router: Router<SharedState>) -> Router<SharedState> {
        router.nest("/process",
                    Router::new()
                        .route("/",post(process))
                        .route("/configs",get(get_configs))
                        .route("/check",post(check_file))
                        .route("/limits",get(get_limits))
                        .route("/test",get(test))
        )
    }
}


async fn test() -> ResultUtil{
    ResultUtil::success(String::from("测试成功"))
}

async fn get_configs(State(state):State<SharedState>) -> ResultUtil<Vec<ConfigsResponse>>{
    ResultUtil::success_with_data(String::from("获取配置文件成功"),ConfigsResponse::from_server_config(&state.server_config))
}

async fn process(State(state):State<SharedState>,Json(process): Json<ProcessRequest>) -> ResultUtil{
    if process.config_index >= state.server_config.output.len(){
        return ResultUtil::fail(String::from("选择的配置文件不存在"));
    }

    let config = &state.server_config.output[process.config_index];
    match handle_unzip_file(Path::new(&process.path),&process.name,config){
        Ok(_) => ResultUtil::success(String::from("处理文件完成")),
        Err(e) => ResultUtil::fail(e.to_string())
    }
}


// 检查文件是否可用
async fn check_file(State(state):State<SharedState>,Json(check): Json<CheckRequest>) -> ResultUtil<bool>{
    if check.config_index >= state.server_config.output.len(){
        return ResultUtil::fail_with_data(String::from("选择的配置文件不存在"),false);
    }

    let config = &state.server_config.output[check.config_index];
    ResultUtil::success_with_data(String::from("请求校验文件成功"),match check_file_exists(config, &check.name) {
        Ok(_) => true,
        Err(_) => false
    })
}

// 获取可选的配置后缀
// todo 后续需要优化ResultUtil类，两个泛型，success跟fail泛型可以不一样
async fn get_limits(State(state):State<SharedState>,Query(limit):Query<LimitsQuery>) -> ResultUtil<Vec<String>>{
    let config_index = limit.config_index;
    if config_index >= state.server_config.output.len(){
        return ResultUtil::fail_with_data(String::from("选择的配置文件不存在"),vec![]);
    }

    ResultUtil::success_with_data(String::from("获取可选的配置后缀成功"),state.server_config.output[config_index].format_limit.clone())
}

