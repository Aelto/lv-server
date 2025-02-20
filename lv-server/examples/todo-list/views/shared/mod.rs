use lv_server::deps::actix_web;

mod fragments;
pub use fragments::Header;

pub fn fragments_router(cfg: &mut actix_web::web::ServiceConfig) {
  use lv_server::WithRouter;

  Header::router(cfg);
}
