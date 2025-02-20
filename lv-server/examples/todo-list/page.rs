use crate::prelude::*;

/// Main template for a complete HTML page, uses the provided [Markup] as the body.
/// The header and footers are automatically added around that body.
pub fn page(content: Markup) -> Markup {
  html!(
    (maud::DOCTYPE)
    html lang="en" {
      head
        // one of the many layers to protect against CSRF, this one is mandatory
        // in order to sent any request that's not a GET towards a fragment.
        data-csrf={(nanoid::nanoid!())}
      {
        meta charset="utf-8";
        title { "lv_server" }
        script type="text/javascript" src="/static/htmx.min.js" {}
        script type="text/javascript" src="/static/main.js" {}
        link rel="stylesheet" href="/static/style.css";
        meta name="htmx-config" content="{\"defaultSwapStyle\":\"outerHTML\", \"selfRequestsOnly\": true}";

      }
      body {
        (crate::views::shared::Header::render())
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
    div id="lv-alert" hidden {}
  )
}
