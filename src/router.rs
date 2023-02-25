use axum::{
    routing::{get, IntoMakeService},
    Router,
};

fn root_router() -> Router {
    Router::new().route("/", get(|| async { "Hello World" }))
}

pub fn router_service() -> IntoMakeService<Router> {
    root_router().into_make_service()
}
