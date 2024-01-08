use crate::WithRouter;

/// Represents the core of the page, which can be composed of as many
/// [Fragment](super::Fragment) as needed to render completely.
pub trait View<Fragments>: WithRouter
where
  Fragments: WithRouter
{
  /// Unlike the [WithRouter] trait this function comes from the [View] trait
  /// with a default implementation that combines the fragments and the View's
  /// render function.
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .configure(<Self as WithRouter>::router)
      .configure(Fragments::router);
  }
}
