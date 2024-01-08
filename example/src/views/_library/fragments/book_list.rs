use crate::prelude::*;

pub struct BookList;

lv_server::endpoints!(BookList {
  get_library_book_list => GET "libraries/{library_id}/book-list"
});

pub enum BookListEvents {
  Reload
}

impl api::get_library_book_list::Router {
  pub async fn endpoint(Need(PELibrary(library)): Need<PELibrary>) -> HttpResponse {
    let books = library.books().unwrap();
    let view = BookList::render(&library.id, &books);

    lv_server::responses::html(view)
  }
}

impl lv_server::Fragment<BookListEvents, api::Router> for BookList {
  const ID: &'static str = "BookList";
}
impl lv_server::WithTrigger for BookListEvents {
  fn into_trigger(self) -> &'static str {
    match self {
      Self::Reload => "fetch-book-list"
    }
  }
}

impl BookList {
  pub fn render(library_id: &String, books: &Vec<Book>) -> Markup {
    html!(
      div.books
        hx-get={(api::get_library_book_list::url(&library_id))}
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
