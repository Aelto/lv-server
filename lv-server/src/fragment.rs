use crate::{WithRouter, WithTrigger};

pub trait Fragment<Events, Endpoints>
where
  Events: WithTrigger,
  Endpoints: WithRouter
{
  const ID: &'static str;

  fn url(path: &str) -> String {
    let slash = match path.starts_with("/") {
      true => "",
      false => "/"
    };

    format!("/frg/{}{slash}{}", Self::ID, path)
  }

  /// Create a route for the fragment with a prefix generated from [Fragment::identifier]
  fn fragment_route(
    cfg: &mut actix_web::web::ServiceConfig, path: &'static str, route: actix_web::Route
  ) {
    cfg.route(&Self::url(path), route);
  }
}
