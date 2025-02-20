use crate::prelude::*;

/// Using a fragment for rendering a static header is definitely too much,
/// however this serves as an example of what a basic fragment looks like.
///
/// Having this element as a fragment allows us to expand it later on, for example
/// when authentication is added and we need to display the dynamic username.
pub struct Header;

impl lv_server::Fragment<(), api::Router> for Header {
  const ID: &'static str = "Header";
}

lv_server::endpoints!(Header {});

impl Header {
  pub fn render() -> Markup {
    html!(
      header.fdn.row {
        h1 {"lv-server"}
        h2 {"todo-list"}
      }
    )
  }
}
