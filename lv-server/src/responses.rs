use actix_web::{
  http::header::{HeaderName, HeaderValue},
  HttpResponse
};

pub fn as_html(body: &impl maud::Render) -> HttpResponse {
  html(body.render())
}

pub fn html(body: maud::Markup) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html")
    .body(body.into_string())
}

pub fn trigger(mut res: HttpResponse, event: &'static str) -> HttpResponse {
  let headers = res.headers_mut();

  headers.append(
    HeaderName::from_static("hx-trigger"),
    HeaderValue::from_static(event)
  );

  res
}
