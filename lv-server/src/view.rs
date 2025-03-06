use crate::WithRouter;

/// Views represent the core of the pages, which can be composed of as many
/// [Fragment](super::Fragment) as needed to render completely.
///
///  ```rs
/// pub mod fragments;
///
/// pub struct ViewHome;
///
/// impl lv_server::View<(fragments::TodoList, fragments::AddTodoForm)> for ViewHome {}
///
/// lv_server::endpoints!(ViewHome as view {
///   get_index => GET "/"
/// });
///
/// impl api::get_index::Router {
///   async fn endpoint(data: ApiData) -> HttpResponse {
///     page(ViewHome::render(data)).into_response()
///   }
/// }
///
/// impl ViewHome {
///   fn render(data: ApiData) -> Markup {
///     html!(
///       .fdn.col.justify-center.items.center {
///         (fragments::TodoList::render(&data.todos()))
///         (fragments::AddTodoForm::render())
///       }
///     )
///   }
/// }
/// ```
///
/// Views are the accessible pages of the website. A single view can have
/// multiple routes/endpoints, however for smaller but more dynamic
///  changes it is recommended to define fragments on the view:
/// ```rs
/// impl lv_server::View<(fragments::TodoList, fragments::AddTodoForm)> for ViewHome {}
/// ```
///
/// If the view does not have any fragment then a `()` can be used instead.
/// ```rs
/// impl View<()> for MyViewWithoutFragments {
///   // ...
/// }
/// ```
///
/// Linking a fragment to a view tells lv-server to automatically setup an
/// API endpoint for that fragment as soon as the view itself is setup.
///
/// You don't have to worry about how or when to declare the URLs of your fragments as long as you link them to a view.
///
/// ```rs
/// // setting up a view in the main Actix app:
/// fn routes(cfg: &mut actix_web::web::ServiceConfig) {
///   use lv_server::View;
///
///   // this sets up the View itself, but also any fragment it may have:
///   views::ViewHome::router(cfg);
/// }
/// ```
///
/// While [Fragments](super::Fragment) are expected as the first generic
/// type, anything that implements that implements the [WithRouter](super::WithRouter) trait will work, allowing you to append any type
/// you may want to the view's router.
pub trait View<Fragments>: WithRouter
where
  Fragments: WithRouter
{
  /// A default implementation that can be used by external types to get the url
  /// for an endpoint from this View. Can be implemented in order to add a custom
  /// prefix to all endpoints.
  ///
  fn url(path: &str) -> String {
    path.to_owned()
  }

  /// Create a route for the view
  fn view_route(
    cfg: &mut actix_web::web::ServiceConfig, path: &'static str, route: actix_web::Route
  ) {
    cfg.route(&Self::url(path), route);
  }

  /// Unlike the [WithRouter] trait this function comes from the [View] trait
  /// with a default implementation that combines the fragments and the View's
  /// render function.
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .configure(<Self as WithRouter>::router)
      .configure(Fragments::router);
  }
}
