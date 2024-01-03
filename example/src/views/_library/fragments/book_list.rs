use crate::prelude::*;

pub struct BookList;
pub enum BookListEvents {
  Reload
}

impl lv_server::Fragment<BookListEvents> for BookList {
  const ID: &'static str = "BookList";
}
impl lv_server::WithTrigger for BookListEvents {
  fn into_trigger(self) -> &'static str {
    match self {
      Self::Reload => "fetch-book-list"
    }
  }
}
impl lv_server::WithRouter for BookList {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    BookList::fragment_route(
      cfg,
      "libraries/{lib}/book-list",
      get().to(get_library_book_list)
    );

    async fn get_library_book_list(
      Need(LibraryPathExt(library)): Need<LibraryPathExt>
    ) -> HttpResponse {
      let books = library.books().unwrap();
      let view = BookList::render(&library.id, &books);

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
