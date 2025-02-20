use proc_macro::TokenStream;

mod endpoints;

/// # Example
/// ```rs
/// // can also extend a route with custom config
/// impl ProjectEditForms {
///   fn extend_config_limit(
///     cfg: &mut actix_web::web::ServiceConfig
///   ) -> &mut actix_web::web::ServiceConfig {
///     cfg.app_data(web::FormConfig::default().limit(4096))
///   }
/// }
///
/// lv_server::endpoints!(ProjectEditForms {
///   get_index => GET "{account_id}/{project_slug}"
///   post_create_form => POST "create"
///   post_edit_form extend(extend_config_limit) => POST "{account_id}/{project_slug}"
///   delete_project => DELETE "{account_id}/{project_slug}"
/// });
/// ```
#[proc_macro]
pub fn endpoints(input: TokenStream) -> TokenStream {
  let content = input.to_string();
  let (_, model) = endpoints::Router::parse(&content).unwrap_or_default();
  let output = model.to_string();

  // use the following to debug outputs
  // eprintln!("{output}");

  use std::str::FromStr;
  TokenStream::from_str(&output).unwrap_or_default()
}

mod events;

/// # Example
///
/// Declaring the list of events:
/// ```rs
/// lv_server::events!(ProjectEditFormsEvents {
///   Reload "from:body"
/// });
/// ```
///
///
/// Reacting to the events:
/// ```rs
/// div
///   hx-trigger={(ProjectEditFormsEvents::Reload)}
///   hx-get={(api::get_index::url())}
///   hx-target="this"
///   {"This div sends a GET request on this event"}
/// ```
///
///
/// Activating the event from an API endpoint:
/// ```rs
/// impl api::post_index::Router {
///   pub async fn endpoint() -> HttpResponse {
///     ProjectEditForms::render()
///       .join(lv_server::responses::alert("success", "this is a success alert!"))
///       .into_response_with_event(ProjectEditFormsEvents::Reload)
///   }
/// }
/// ```
#[proc_macro]
pub fn events(input: TokenStream) -> TokenStream {
  let content = input.to_string();
  let (_, events) = events::Events::parse(&content).unwrap_or_default();
  let output = events.to_string();

  // use the following to debug outputs
  // eprintln!("{output}");

  use std::str::FromStr;
  TokenStream::from_str(&output).unwrap_or_default()
}

mod prelude {

  pub use nom::bytes::complete::{tag, take_until1, take_while, take_while1};
  pub use nom::error::ParseError;
  pub use nom::multi::many0;
  pub use nom::IResult;

  pub fn trim(i: &str) -> IResult<&str, &str> {
    take_while(|c| c == ' ' || c == '\n' || c == '\r')(i)
  }
}
