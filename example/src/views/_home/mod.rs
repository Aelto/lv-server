use crate::prelude::*;

mod fragments;

pub struct ViewHome;
impl lv_server::View<fragments::AuthorList> for ViewHome {}

impl lv_server::WithRouter for ViewHome {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/", get().to(index));

    async fn index() -> HttpResponse {
      lv_server::responses::html(page(html!(
        div
          hx-get={(fragments::author_list::api::get_author_list::url())}
          hx-trigger="load"
          {}
      )))
    }
  }
}
