use crate::{WithRouter, WithTrigger};

pub trait Fragment<Events, Endpoints>
where
  Events: WithTrigger,
  Endpoints: WithRouter
{
  const ID: &'static str;

  fn url(path: &str) -> String {
    format!("/frg/{}/{}", Self::ID, path.trim_start_matches('/'))
  }

  /// Create a route for the fragment with a prefix generated from [Fragment::identifier]
  fn fragment_route(
    cfg: &mut actix_web::web::ServiceConfig, path: &'static str, route: actix_web::Route
  ) {
    cfg.route(&Self::url(path), route);
  }
}
