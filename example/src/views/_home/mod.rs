use crate::prelude::*;

mod fragments;

pub struct ViewHome;
impl lv_server::View<(fragments::AuthorList, fragments::RecentArticleList)> for ViewHome {}

lv_server::endpoints!(ViewHome as view {
  get_index => GET "/"
});

impl api::get_index::Router {
  async fn endpoint() -> HttpResponse {
    lv_server::responses::html(page(html!(

      h2 {"Recent articles"}
      div
        hx-get={(fragments::recent_article_list::api::get_list::url())}
        hx-trigger="load"
        {}

      h2 {"All authors"}
      div
        hx-get={(fragments::author_list::api::get_author_list::url())}
        hx-trigger="load"
        {}
    )))
  }
}

// impl lv_server::WithRouter for ViewHome {
//   fn router(cfg: &mut actix_web::web::ServiceConfig) {
//     cfg.route("/", get().to(index));

//     async fn index() -> HttpResponse {
//       lv_server::responses::html(page(html!(

//         h2 {"Recent articles"}
//         div
//           hx-get={(fragments::recent_article_list::api::get_list::url())}
//           hx-trigger="load"
//           {}

//         h2 {"All authors"}
//         div
//           hx-get={(fragments::author_list::api::get_author_list::url())}
//           hx-trigger="load"
//           {}
//       )))
//     }
//   }
// }
