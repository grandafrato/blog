pub mod index;

use askama::Template;
use sycamore::render_to_string;
use sycamore::web::SsrNode;
use sycamore::{reactive::Scope, view::View};

#[derive(Template)]
#[template(path = "root.html", escape = "none")]
struct RootHtmlTemplate<'a> {
    body: &'a str,
    title: &'a str,
}

pub fn root_html(body: &dyn Fn(Scope) -> View<SsrNode>, title: &str) -> String {
    let template = RootHtmlTemplate {
        body: &render_to_string(body),
        title,
    };
    template.render().expect("Unable to render root template.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;
    use sycamore::view;

    #[test]
    fn test_root_html_returns_html_root() {
        let html = root_html(&|cx| view! { cx, ""}, "");

        assert_eq!(
            html,
            r#"<!DOCTYPE html>
<html lang="en">

<head>
  <title></title>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link href="/assets/css/style.css" rel="stylesheet">
</head>

<body>
  
</body>

</html>"#
        );
    }

    proptest! {
        #[test]
        fn test_root_html_includes_provided_title(title in "\\PC*") {
            let html = root_html(&|cx| view! { cx, "" }, &title);

            assert_eq!(
                html,
                format!(r#"<!DOCTYPE html>
<html lang="en">

<head>
  <title>{}</title>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link href="/assets/css/style.css" rel="stylesheet">
</head>

<body>
  
</body>

</html>"#, title)
            );
        }
    }

    proptest! {
        #[test]
        fn test_root_html_includes_provided_body(body in "\\PC*") {
            let b1 = body.clone();
            let html = root_html(&(move |cx| { view! { cx,  }}), "");

            assert_eq!(
                html,
                format!(r#"<!DOCTYPE html>
<html lang="en">

<head>
  <title></title>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link href="/assets/css/style.css" rel="stylesheet">
</head>

<body>
  {}
</body>

</html>"#, b1)
        );
        }
    }
}
