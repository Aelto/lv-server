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
        (crate::fragments::Header::render())
        (content)
        (alerts())
      }
    }
  )
}

/// Sets up the basic nodes for the alerts, htmx then handles the swapping of
/// this node with toast notifications.
fn alerts() -> Markup {
  html!(
    div id="alerts" class="hidden" {}
  )
}
