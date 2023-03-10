use crate::page::{index::Index, root_html};
use axum::response::Html;

pub async fn index() -> Html<String> {
    root_html(&Index, "Lachlan's Blog").into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use pretty_assertions::assert_eq;
    use scraper::{Html as ScraperHtml, Selector};

    #[tokio::test]
    async fn test_index() {
        let raw_html = String::from_utf8(
            hyper::body::to_bytes(index().await.into_response().into_body())
                .await
                .unwrap()
                .into(),
        )
        .unwrap();

        let html = ScraperHtml::parse_document(&raw_html);
        let selector = Selector::parse("title").unwrap();

        assert_eq!(
            "Lachlan's Blog",
            html.select(&selector).next().unwrap().inner_html()
        );
    }
}
