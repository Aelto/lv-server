pub fn csrf_protection(
  route: actix_web::Route, method: actix_web::http::Method
) -> actix_web::Route {
  match method {
    actix_web::http::Method::GET => route,
    _ => csrf_header_guard(route)
  }
}

pub fn is_csrf_safe(req: &actix_web::HttpRequest) -> bool {
  has_csrf_header(req.head().headers())
}

fn has_csrf_header(headers: &actix_web::http::header::HeaderMap) -> bool {
  headers.contains_key("X-LVSERVER-REQ")
}

fn csrf_header_guard(route: actix_web::Route) -> actix_web::Route {
  route.guard(actix_web::guard::fn_guard(|c| {
    has_csrf_header(c.head().headers())
  }))
}
