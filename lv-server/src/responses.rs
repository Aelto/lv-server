//! This module contains a set of utility functions to quickly send valid
//! Actix responses from the endpoints.
use actix_web::http::header::{HeaderName, HeaderValue};

pub use actix_web::HttpResponse;

/// Sends an HttpResponse with an empty HTML body inside it
pub fn empty_html() -> HttpResponse {
  html(maud::html!())
}

/// Sends an HttpResponse with supplied body, as long as it implements maud's [Render](maud::Render) trait.
pub fn as_html(body: &impl maud::Render) -> HttpResponse {
  html(body.render())
}

/// Sends an HttpResponse with supplied body, as long as it implements maud's [Render](maud::Render) trait.
pub fn html(body: maud::Markup) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body(body.into_string())
}

/// Sends a completely empty HttpResponse with no Body
pub fn no_content() -> HttpResponse {
  HttpResponse::NoContent().finish()
}

/// Adds a HX-Redirect header to the response to perform a front-end redirect
/// once the response is received.
pub fn redirect(mut res: HttpResponse, target_url: &str) -> HttpResponse {
  use std::str::FromStr;

  if let Ok(value) = HeaderValue::from_str(target_url) {
    // HeaderName::from_static(str) doesn't accept headers with uppercase letters
    if let Ok(name) = HeaderName::from_str("HX-Redirect") {
      let headers = res.headers_mut();

      headers.append(name, value);
    }
  }

  res
}

/// Modifies the supplied HttpResponse to append it a hx-trigger header for
/// the given event.
///
/// Can be used directly if needed, but using the [endpoints!](crate::endpoints) macro might offer better ergonomics.
pub fn trigger(mut res: HttpResponse, event: &'static str) -> HttpResponse {
  let headers = res.headers_mut();

  headers.append(
    HeaderName::from_static("hx-trigger"),
    HeaderValue::from_static(event)
  );

  res
}

/// Send an oob alert message that can be used as a quick way to bubble up
/// errors to the front-end.
///
/// # Setting up
/// For the errors to appear correctly on the page an anchor div must be placed
/// in the core layout:
/// ```html
/// <div id="lv-alert" hidden ></div>
/// ```
/// The errors created by this function will then replace the anchor as they
/// appear.
pub fn alert(class: &str, message: &impl maud::Render) -> maud::Markup {
  maud::html!(
    div id="lv-alert" class={(class)} hx-swap-oob="true" onclick="this.setAttribute('hidden', 'hidden')" {
      (message)
    }
  )
}
