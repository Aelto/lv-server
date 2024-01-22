use crate::prelude::*;

#[derive(Debug)]
pub struct AddLibraryButton;

lv_server::endpoints!(AddLibraryButton {
  get_button => GET "authors/{author_id}/button"
  get_form => GET "authors/{author_id}/form"
  post_library => POST "authors/{author_id}/libraries"
});

impl api::get_button::Router {
  pub async fn endpoint(Need(author): Need<Author>) -> AppResponse {
    Ok(lv_server::responses::html(AddLibraryButton::render_button(
      author.id_res()?
    )))
  }
}

impl api::get_form::Router {
  pub async fn endpoint(Need(author): Need<Author>) -> AppResponse {
    Ok(lv_server::responses::html(AddLibraryButton::render_form(
      author.id_res()?
    )))
  }
}

#[derive(Deserialize)]
pub(super) struct FormPostLibrary {
  title: String
}
impl api::post_library::Router {
  pub async fn endpoint(Need(author): Need<Author>, form: Form<FormPostLibrary>) -> AppResponse {
    let form = form.into_inner();
    let _ = Library {
      title: form.title,
      ..Default::default()
    }
    .create(&author.id);

    let view = AddLibraryButton::render_button(&author.id);
    let res = lv_server::responses::html(view);

    Ok(super::AuthorLibraryListEvents::Reload.trigger(res))
  }
}

impl lv_server::Fragment<(), api::Router> for AddLibraryButton {
  const ID: &'static str = "AddLibraryButton";
}

impl AddLibraryButton {
  pub fn render_button(author_id: &Id) -> Markup {
    html!(
      button
        hx-get={(api::get_form::url(author_id.id()))}
        hx-target="this"
        hx-swap="outerHTML"
        {"Add library"}
    )
  }

  fn render_form(author_id: &Id) -> Markup {
    html!(
      form
        hx-post={(api::post_library::url(author_id.id()))}
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
