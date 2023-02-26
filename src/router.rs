use crate::controllers;
use axum::{
    routing::{get, get_service, IntoMakeService},
    Router,
};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

fn page_router() -> Router {
    Router::new().route("/", get(controllers::index))
}

fn asset_router() -> Router {
    Router::new().nest_service("/assets", get_service(ServeDir::new("assets")))
}

fn root_router() -> Router {
    Router::new()
        .merge(page_router())
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
    use pretty_assertions::assert_eq;
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

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let file_body = fs::read("assets/css/style.css").await.unwrap();

        assert_eq!(response_body, file_body);
    }

    #[tokio::test]
    async fn test_page_routes_have_html_root() {
        let app = page_router();

        // For now these routes are hard-coded, I'm looking for a way to automatically
        // get all the page routes.
        let page_routes = ["/"];

        for route in page_routes {
            let response = app
                .clone()
                .oneshot(Request::builder().uri(route).body(Body::empty()).unwrap())
                .await
                .unwrap();

            let headers = response.headers();

            assert_eq!(response.status(), StatusCode::OK);
            assert!(headers.contains_key("Content-Type"));
            assert_eq!(
                headers.get("Content-Type").unwrap(),
                &"text/html; charset=utf-8"
            );

            let body = String::from_utf8(
                hyper::body::to_bytes(response.into_body())
                    .await
                    .unwrap()
                    .into(),
            )
            .unwrap();

            // Since we're just checking that the beginning and end of the response, we are just
            // ignoring everything in the body tag.
            let (first_part_of_html, second_part_of_html) = body.split_once("<body>").unwrap();
            let (_, second_part_of_html) = second_part_of_html.split_once("</body>").unwrap();

            assert_eq!(
                first_part_of_html,
                r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <title></title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link href="/assets/css/style.css" rel="stylesheet">
  </head>
  "#
            );
            assert_eq!(second_part_of_html, "\n</html>");
        }
    }
}
