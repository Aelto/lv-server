use actix_web::{
  http::header::{HeaderName, HeaderValue},
  HttpResponse
};

pub fn empty_html() -> HttpResponse {
  html(maud::html!())
}

pub fn as_html(body: &impl maud::Render) -> HttpResponse {
  html(body.render())
}

pub fn html(body: maud::Markup) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body(body.into_string())
}

pub fn no_content() -> HttpResponse {
  HttpResponse::NoContent().finish()
}

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
pub fn alert(message: &impl maud::Render) -> maud::Markup {
  maud::html!(
    div id="lv-alert" hx-swap-oob="true" onclick="this.setAttribute('hidden', 'hidden')" {
      (message)
    }
  )
}
