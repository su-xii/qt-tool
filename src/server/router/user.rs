mod dto;

use crate::server::app_state::SharedState;
use crate::server::router::user::dto::response::UserResponse;
use crate::server::router::AppRouter;
use crate::util::result_util::ResultUtil;
use axum::{routing::get, Json, Router};
use axum::routing::post;
use crate::server::router::user::dto::request::UserRequest;

pub struct UserRouter;

impl AppRouter<SharedState> for UserRouter {
    fn create_router(&self,router: Router<SharedState>) -> Router<SharedState> {
        router.nest("/user",
                    Router::new()
                        .route("/",get(get_user))
                        .route("/get",post(get_user2))
        )
    }
}

async fn get_user() -> ResultUtil<UserResponse>{
    ResultUtil::success_with_data("请求成功".to_string(),UserResponse{
        name: "hhhh".to_string(),
        id: 0,
    })
}

async fn get_user2(Json(user): Json<UserRequest>) -> ResultUtil<UserResponse>{
    ResultUtil::success_with_data("请求成功".to_string(),UserResponse{
        name: "hhhh".to_string(),
        id: user.id,
    })
}
