mod dto;

use std::path::Path;
use axum::extract::{Multipart, Query, State};
use dto::ConfigsResponse;
use crate::server::app_state::{SharedState};
use crate::server::router::AppRouter;
use crate::util::result_util::ResultUtil;
use axum::routing::{get, post};
use axum::{Json, Router};
use crate::handler::{check_file_exists, handle_unzip_file};
use crate::server::router::process::dto::{CheckRequest, LimitsQuery, ProcessRequest};
use uuid::Uuid;
use tokio::io::AsyncWriteExt;


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
                        .route("/upload",post(upload_file))
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

async fn upload_file(State(state): State<SharedState>, mut multipart: Multipart) -> ResultUtil {

    let mut name: Option<String> = None;
    let mut config_index: Option<usize> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut original_filename: Option<String> = None;

    // 解析 multipart 字段
    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "name" => {
                name = Some(field.text().await.unwrap_or_default());
            }
            "config_index" => {
                let text = field.text().await.unwrap_or_default();
                config_index = text.parse::<usize>().ok();
            }
            "file" => {
                original_filename = field.file_name().map(|s| s.to_string());
                file_data = field.bytes().await.ok().map(|b| b.to_vec());
            }
            _ => {}
        }
    }

    // 校验必要字段
    let name = match name {
        Some(n) if !n.is_empty() => n,
        _ => return ResultUtil::fail(String::from("缺少文件名参数")),
    };
    let config_index = match config_index {
        Some(i) => i,
        None => return ResultUtil::fail(String::from("缺少配置索引参数")),
    };
    let file_data = match file_data {
        Some(d) if !d.is_empty() => d,
        _ => return ResultUtil::fail(String::from("缺少上传文件")),
    };

    // 校验配置索引
    if config_index >= state.server_config.output.len() {
        return ResultUtil::fail(String::from("选择的配置文件不存在"));
    }

    // 确定文件扩展名
    let ext = original_filename
        .as_deref()
        .and_then(|f| Path::new(f).extension())
        .and_then(|e| e.to_str())
        .unwrap_or("zip");

    // 生成 UUID 文件名，保存到 update/ 目录
    let update_dir = std::env::current_dir()
        .unwrap_or_else(|_| Path::new(".").to_path_buf())
        .join("update");
    if let Err(e) = tokio::fs::create_dir_all(&update_dir).await {
        return ResultUtil::fail(format!("创建上传目录失败: {}", e));
    }

    let unique_filename = format!("{}.{}", Uuid::new_v4(), ext);
    let file_path = update_dir.join(&unique_filename);

    // 异步写入文件
    if let Err(e) = async {
        let mut file = tokio::fs::File::create(&file_path).await?;
        file.write_all(&file_data).await?;
        file.flush().await?;
        Ok::<_, std::io::Error>(())
    }.await {
        return ResultUtil::fail(format!("保存文件失败: {}", e));
    }

    // 获取相对路径字符串
    let relative_path = file_path.to_string_lossy().to_string();

    tracing::info!("上传文件已保存: {} (原始: {:?})", relative_path, original_filename);

    // 复用现有的解压/切图逻辑
    let config = &state.server_config.output[config_index];
    let result = handle_unzip_file(Path::new(&relative_path), &name, config);

    // 处理完成后异步清理临时文件
    if let Err(e) = tokio::fs::remove_file(&file_path).await {
        tracing::warn!("清理临时文件失败: {} -> {}", relative_path, e);
    }

    match result {
        Ok(_) => ResultUtil::success(String::from("处理文件完成")),
        Err(e) => ResultUtil::fail(e.to_string()),
    }
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

