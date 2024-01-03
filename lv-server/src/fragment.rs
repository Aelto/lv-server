use crate::{WithRouter, WithTrigger};

pub trait Fragment<Events>: WithRouter
where
  Events: WithTrigger
{
  const ID: &'static str;

  fn url(path: &str) -> String {
    format!("/frg/{}/{}", Self::ID, path)
  }

  /// Create a route for the fragment with a prefix generated from [Fragment::identifier]
  ///
  /// The provided [`path`] must **NOT** start with a `/`
  fn fragment_route(
    cfg: &mut actix_web::web::ServiceConfig, path: &'static str, route: actix_web::Route
  ) {
    assert!(
      !path.starts_with("/"),
      "Provided path must not start with a /"
    );

    cfg.route(&Self::url(path), route);
  }
}
