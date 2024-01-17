use crate::prelude::*;

pub struct AddBookButton;

impl lv_server::Fragment<(), api::Router> for AddBookButton {
  const ID: &'static str = "AddBookButton";
}

lv_server::endpoints!(AddBookButton {
  get_library_add_book_button => GET "libraries/{library_id}/button"
  get_library_add_book_form => GET "libraries/{library_id}/form"
  post_library_book => POST "libraries/{library_id}"
});

impl api::get_library_add_book_button::Router {
  pub async fn endpoint(path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    lv_server::responses::html(AddBookButton::render(&id))
  }
}

impl api::get_library_add_book_form::Router {
  pub async fn endpoint(Need(library): Need<Library>) -> HttpResponse {
    lv_server::responses::html(AddBookButton::render_form(&library))
  }
}

impl api::post_library_book::Router {
  pub(self) async fn endpoint(
    Need(library): Need<Library>, Form(data): Form<AddBookForm>
  ) -> HttpResponse {
    let fragment = AddBookButton::render(&library.id);
    let book = Book {
      id: String::new(),
      title: data.title,
      content: String::new(),
      fk_library: String::new()
    };
    book.add(library.id).unwrap();

    super::BookListEvents::Reload.trigger(lv_server::responses::html(fragment))
  }
}

#[derive(Serialize, Deserialize)]
struct AddBookForm {
  title: String
}

impl AddBookButton {
  pub fn render(library_id: &String) -> Markup {
    html!(
      button
        hx-target="this"
        hx-swap="outerHTML"
        hx-get={(api::get_library_add_book_form::url(library_id))}
        {"Add book"}
    )
  }

  fn render_form(lib: &Library) -> Markup {
    html!(
      form
        hx-post={(api::post_library_book::url(&lib.id))}
        hx-target="this"
        hx-swap="outerHTML" {

        h2 {"Add a book to "(lib.title)}

        div {
          label {"Title"}
          input type="text" name="title" placeholder="README" value="README";
        }

        div {
          button {"Create book"}
          button
            hx-get={(api::get_library_add_book_button::url(&lib.id))} {"Cancel"}
        }
      }
    )
  }
}
