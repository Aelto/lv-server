use crate::prelude::*;

pub fn page(content: Markup) -> Markup {
  html!(
    (maud::DOCTYPE)
    html lang="en" {
      head {
        meta charset="utf-8";
        title { "lv_server" }
        script src="/static/htmx.min.js" {}
        link href="/static/style.css" rel="stylesheet";
      }
      body {
        a href="/" {"HOME"}

        (content)

      }
    }
  )
}
