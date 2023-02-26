use axum::{
    routing::{get, get_service, IntoMakeService},
    Router,
};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

fn asset_router() -> Router {
    Router::new().nest_service("/assets", get_service(ServeDir::new("assets")))
}

fn root_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .merge(asset_router())
        .layer(TraceLayer::new_for_http())
}

pub fn router_service() -> IntoMakeService<Router> {
    root_router().into_make_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use std::net::{SocketAddr, TcpListener};
    use tokio::fs;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_asset_router_returns_files_under_assets() -> Result<(), walkdir::Error> {
        use walkdir::WalkDir;

        let app = asset_router();

        for entry in WalkDir::new("assets") {
            let entry = entry?;
            let file_metadata = entry.metadata()?;

            if file_metadata.is_file() {
                let response = app
                    .clone()
                    .oneshot(
                        Request::builder()
                            .uri(format!("/{}", entry.path().to_str().unwrap()))
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                assert_eq!(response.status(), StatusCode::OK);

                let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
                let file_body = fs::read(entry.path()).await.unwrap();

                assert_eq!(response_body, file_body);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_server_returns_stylesheet() {
        let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(router_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/assets/css/style.css", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let file_body = fs::read("assets/css/style.css").await.unwrap();

        assert_eq!(response_body, file_body);
    }
}
