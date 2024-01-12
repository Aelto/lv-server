use crate::prelude::*;

pub struct Header;

lv_server::endpoints!(Header {});

impl lv_server::Fragment<(), api::Router> for Header {
  const ID: &'static str = "Header";
}

impl Header {
  pub fn render() -> Markup {
    html!(
      header {
        h1 {a href="/" {"Books"}}
      }
    )
  }
}
