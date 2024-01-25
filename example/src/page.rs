use crate::prelude::*;

pub fn page(content: Markup) -> Markup {
  html!(
    (maud::DOCTYPE)
    html lang="en" {
      head
        data-csrf={(nanoid::nanoid!())} {

        meta charset="utf-8";
        title { "lv_server" }
        script type="text/javascript" src="/static/htmx.min.js" {}
        script type="text/javascript" src="/static/main.js" {}
        link href="/static/style.css" rel="stylesheet";
        meta name="htmx-config" content="{\"defaultSwapStyle\":\"outerHTML\", \"selfRequestsOnly\": true}";
        meta http-equiv="Content-Security-Policy" content="default-src 'self';";
      }
      body {
        (crate::fragments::Header::render())
        div id="content" {(content)}
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
