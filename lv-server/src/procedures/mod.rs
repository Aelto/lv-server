use actix_web::web::Path;

pub mod registry;

pub fn add_procedures_handler(cfg: &mut actix_web::web::ServiceConfig) {
  async fn handler(request: actix_web::HttpRequest, body: actix_web::web::Payload, path: Path<String>) -> actix_web::HttpResponse {
    let id = path.into_inner();
    let lock = registry::HANDLER_REGISTRY.lock().unwrap();
    if let Some(handler) = lock.get(&id) {
      let response = handler(request, body);

      return response.await
    }

    crate::responses::no_content()
  }
  cfg.route("/lvsrv/procs/{id}", actix_web::web::post().to(handler));
}
