use sycamore::{component, view};
use sycamore::{reactive::Scope, view::View, web::Html};

#[component]
pub fn Index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        article(class="prose") {
            h1 { "Index" }
            p { "Hello" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use scraper::{Html, Selector};
    use selectors::attr::CaseSensitivity;
    use std::error::Error;
    use sycamore::{render_to_string, view};

    #[test]
    fn test_index_contains_one_article() -> Result<(), Box<dyn Error>> {
        let index_output = render_to_string(|cx| view! {cx, Index()});

        let html = Html::parse_fragment(&index_output);
        let element_selector = Selector::parse("article")?;

        assert_eq!(html.select(&element_selector).count(), 1);

        Ok(())
    }

    #[test]
    fn test_index_article_is_styled_with_prose() -> Result<(), Box<dyn Error>> {
        let index_output = render_to_string(|cx| view! {cx, Index()});

        let html = Html::parse_fragment(&index_output);
        let element_selector = Selector::parse("article")?;
        let element = html.select(&element_selector).next().unwrap().value();

        assert!(element.has_class("prose", CaseSensitivity::CaseSensitive));

        Ok(())
    }
}
