use crate::prelude::*;

pub struct BookList;
pub enum BookListEvents {
  Reload
}

impl lv_server::Fragment<BookListEvents> for BookList {}
impl lv_server::WithTrigger for BookListEvents {
  fn into_trigger(self) -> &'static str {
    match self {
      Self::Reload => "fetch-book-list"
    }
  }
}
impl lv_server::WithRouter for BookList {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route(
      "/frg/BookList/libraries/{lib}/book-list",
      get().to(get_library_book_list)
    );

    async fn get_library_book_list(path: Path<String>) -> HttpResponse {
      let id = path.into_inner();

      let Some(library) = Library::find_by_id(&id).unwrap() else {
        return lv_server::responses::as_html(&"no library with this id");
      };

      let books = library.books().unwrap();
      let view = BookList::render(&id, &books);

      lv_server::responses::html(view)
    }
  }
}

impl BookList {
  pub fn render(library_id: &String, books: &Vec<Book>) -> Markup {
    html!(
      div.books
        hx-get={"/frg/BookList/libraries/"(library_id)"/book-list"}
        hx-trigger={"fetch-book-list from:body"} {

        ul {
          @for book in books {
            li {
              a href={"?book="(book.id)} {(book.title)}
            }
          }
        }

      }
    )
  }
}
