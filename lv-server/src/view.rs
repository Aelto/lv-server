use crate::WithRouter;

/// Represents the core of the page, which can be composed of as many
/// [Fragment](super::Fragment) as needed to render completely.
pub trait View<Fragments>: WithRouter
where
  Fragments: WithRouter
{
  /// A default implementation that can be used by external types to get the url
  /// for an endpoint from this View. Can be implemented in order to add a custom
  /// prefix to all endpoints.
  ///
  fn url(path: &str) -> String {
    path.to_owned()
  }

  /// Create a route for the view
  fn view_route(
    cfg: &mut actix_web::web::ServiceConfig, path: &'static str, route: actix_web::Route
  ) {
    cfg.route(&Self::url(path), route);
  }

  /// Unlike the [WithRouter] trait this function comes from the [View] trait
  /// with a default implementation that combines the fragments and the View's
  /// render function.
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .configure(<Self as WithRouter>::router)
      .configure(Fragments::router);
  }
}
