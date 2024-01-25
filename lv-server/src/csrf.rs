pub fn csrf_protection(
  route: actix_web::Route, method: actix_web::http::Method
) -> actix_web::Route {
  match dbg!(method) {
    actix_web::http::Method::GET => route,
    _ => csrf_header_guard(route)
  }
}

fn csrf_header_guard(route: actix_web::Route) -> actix_web::Route {
  route.guard(actix_web::guard::fn_guard(|c| {
    dbg!(c.head().headers()).contains_key("X-LVSERVER-REQ")
  }))
}
