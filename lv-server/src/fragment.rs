use crate::{WithRouter, WithTrigger};

/// Fragments are small parts of the [Views](crate::View) that need their
/// own set of endpoints without polluting the view's router. They can be used as components with their own internal logic.
///
/// Fragments are similar to Views except
/// 1) they don't have children fragments like views
/// 2) events can be defined with them.
/// 3) their endpoints are automatically given a prefix to avoid users hitting them by mistakes
///
/// ```rs
/// use crate::prelude::*;
///
/// pub struct AddTodoForm;
///
/// impl lv_server::Fragment<(), api::Router> for AddTodoForm {
///   const ID: &'static str = "AddTodoForm";
/// }
///
/// lv_server::endpoints!(AddTodoForm {
///   get_index => GET "/"
///   post_add_todo => POST "/todos"
/// });
///
/// impl api::get_index::Router {
///   pub async fn endpoint() -> HttpResponse {
///     let view = html!();
///     lv_server::responses::html(view)
///   }
/// }
///
/// #[derive(Deserialize)]
/// pub struct PostAddTodoForm {
///   text: String
/// }
///
/// impl api::post_add_todo::Router {
///   pub async fn endpoint(Form(form): Form<PostAddTodoForm>, data: ApiData) -> HttpResponse {
///     if form.text.trim().is_empty() {
///       return AddTodoForm::render()
///         .join(lv_server::responses::alert(
///           "error",
///           &"You can't add an empty todo"
///         ))
///         .into_response();
///     }
///
///     data.add_todo(form.text);
///
///     AddTodoForm::render().into_response_with_event(super::TodoListEvents::Reload)
///   }
/// }
///
/// impl AddTodoForm {
///   pub fn render() -> Markup {
///     html!(
///       form.fdn.row
///         hx-post={(api::post_add_todo::url())}
///         hx-target="this"
///         hx-swap="outerHTML"
///       {
///         input name="text" placeholder="Todo's text" {}
///         input type="submit" value="Add";
///       }
///     )
///   }
/// }
/// ```
///
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
  fn fragment_route<'a>(
    cfg: &'a mut actix_web::web::ServiceConfig, path: &'static str, route: actix_web::Route
  ) -> &'a mut actix_web::web::ServiceConfig {
    cfg.route(&Self::url(path), route)
  }
}
