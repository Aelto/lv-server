pub trait WithScope {
  fn scope() -> actix_web::Scope;
}
