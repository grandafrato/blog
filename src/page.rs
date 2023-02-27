use askama::Template;

#[derive(Template)]
#[template(path = "root.html", escape = "none")]
struct RootHtmlTemplate<'a> {
    body: &'a str,
}

pub fn root_html(body: &str) -> String {
    let template = RootHtmlTemplate { body };
    template.render().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;

    #[test]
    fn test_root_html_returns_html_root() {
        let html = root_html("");

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
        fn test_root_html_includes_provided_body(body in "\\PC*") {
            let html = root_html(&body);

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

</html>"#, body)
        );

        }
    }
}
