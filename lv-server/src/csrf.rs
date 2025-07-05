/// A guard that is added to all endpoints from the [`endpoints!`] macro. It
/// performs basic checks on non-GET methods to eliminate the most unsafe
/// requests and ensure out of-the-box CSRF protection.
pub fn csrf_protection(
  route: actix_web::Route, method: actix_web::http::Method
) -> actix_web::Route {
  match method {
    actix_web::http::Method::GET => route,
    _ => csrf_header_guard(route)
  }
}

pub fn is_csrf_safe(req: &actix_web::HttpRequest) -> bool {
  let headers = req.head().headers();

  has_csrf_header(headers) && is_fetch_request(headers)
}

fn has_csrf_header(headers: &actix_web::http::header::HeaderMap) -> bool {
  headers.contains_key("X-LVSERVER-REQ")
}

fn is_fetch_request(headers: &actix_web::http::header::HeaderMap) -> bool {
  match headers.get("Sec-Fetch-Site").map(|v| v.to_str()) {
    Some(Ok("same-site")) | Some(Ok("same-origin")) => true,
    _ => false
  }
}

fn csrf_header_guard(route: actix_web::Route) -> actix_web::Route {
  route.guard(actix_web::guard::fn_guard(|c| {
    let headers = c.head().headers();

    has_csrf_header(headers) && is_fetch_request(headers)
  }))
}
