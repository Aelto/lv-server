pub trait WithRouter {
  /// The place where the type defines all of the endpoints it depends on
  fn router(cfg: &mut actix_web::web::ServiceConfig);
}
