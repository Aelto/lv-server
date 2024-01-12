use crate::prelude::*;

#[derive(Debug)]
pub struct AddLibraryButton;

lv_server::endpoints!(AddLibraryButton {
  get_button => GET "authors/{author_id}/button"
  get_form => GET "authors/{author_id}/form"
  post_library => POST "authors/{author_id}/libraries"
});

impl api::get_button::Router {
  pub async fn endpoint(Need(PEAuthor(author)): Need<PEAuthor>) -> HttpResponse {
    lv_server::responses::html(AddLibraryButton::render_button(&author.id))
  }
}

impl api::get_form::Router {
  pub async fn endpoint(Need(PEAuthor(author)): Need<PEAuthor>) -> HttpResponse {
    lv_server::responses::html(AddLibraryButton::render_form(&author.id))
  }
}

#[derive(Deserialize)]
pub(super) struct FormPostLibrary {
  title: String
}
impl api::post_library::Router {
  pub async fn endpoint(
    Need(PEAuthor(author)): Need<PEAuthor>, form: Form<FormPostLibrary>
  ) -> HttpResponse {
    let form = form.into_inner();
    let _ = Library {
      title: form.title,
      ..Default::default()
    }
    .add(author.id.clone());

    let view = AddLibraryButton::render_button(&author.id);
    let res = lv_server::responses::html(view);

    super::AuthorLibraryListEvents::Reload.trigger(res)
  }
}

impl lv_server::Fragment<(), api::Router> for AddLibraryButton {
  const ID: &'static str = "AddLibraryButton";
}

impl AddLibraryButton {
  pub fn render_button(author_id: &str) -> Markup {
    html!(
      button
        hx-get={(api::get_form::url(author_id))}
        hx-target="this"
        hx-swap="outerHTML"
        {"Add library"}
    )
  }

  fn render_form(author_id: &str) -> Markup {
    html!(
      form
        hx-post={(api::post_library::url(author_id))}
        hx-target="this"
        hx-swap="outerHTML"
      {
        label for="title" {"Title"}
        input id="title" name="title" value="Lorem";

        input type="submit";
      }
    )
  }
}
