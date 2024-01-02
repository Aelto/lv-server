use crate::prelude::*;

pub struct AddBookButton;

#[derive(Serialize, Deserialize)]
struct AddBookForm {
  title: String
}

impl lv_server::Fragment<()> for AddBookButton {}
impl lv_server::WithRouter for AddBookButton {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .route(
        "/frg/AddBookButton/libraries/{id}/button",
        get().to(get_library_add_book_button)
      )
      .route(
        "/frg/AddBookButton/libraries/{library_id}/form",
        get().to(get_library_add_book_form)
      )
      .route(
        "/frg/AddBookButton/libraries/{id}",
        post().to(add_library_book)
      );

    async fn get_library_add_book_button(path: Path<String>) -> HttpResponse {
      let id = path.into_inner();

      lv_server::responses::html(AddBookButton::render(&id))
    }

    async fn get_library_add_book_form(LibraryPathExt(library): LibraryPathExt) -> HttpResponse {
      lv_server::responses::html(AddBookButton::render_form(&library))
    }

    async fn add_library_book(path: Path<String>, Form(data): Form<AddBookForm>) -> HttpResponse {
      let id = path.into_inner();

      let Some(_) = Library::find_by_id(&id).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      let fragment = AddBookButton::render(&id);
      let book = Book {
        id: String::new(),
        title: data.title,
        content: String::new(),
        fk_library: String::new()
      };
      book.add(id).unwrap();

      super::BookListEvents::Reload.trigger(lv_server::responses::html(fragment))
    }
  }
}
impl AddBookButton {
  pub fn render(library_id: &String) -> Markup {
    html!(
      button
        hx-target="this"
        hx-swap="outerHTML"
        hx-get={"/frg/AddBookButton/libraries/"(library_id)"/form"}
        {"Add book"}
    )
  }

  fn render_form(lib: &Library) -> Markup {
    html!(
      form
        hx-post={"/frg/AddBookButton/libraries/"(lib.id)}
        hx-target="this"
        hx-swap="outerHTML" {

        h2 {"Add a book to "(lib.title)}

        div {
          label {"Title"}
          input type="text" name="title" placeholder="README" value="README";
        }

        button {"Create book"}
        button
          hx-get={"/frg/AddBookButton/"(lib.id)"/button"} {"Cancel"}
      }
    )
  }
}
