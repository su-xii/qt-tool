mod process;
pub use process::ProcessRouter;

use axum::Router;

pub trait AppRouter<S>{
    fn create_router(&self,router:Router<S>) -> Router<S>;

}

pub fn combine_routers<S>(base: Router<S>, routers: Vec<Box<dyn AppRouter<S>>>) -> Router<S> {
    routers.into_iter().fold(base, |router, r| r.create_router(router))
}