use crate::prelude::*;

pub struct BookList;

lv_server::endpoints!(BookList {
  get_library_book_list => GET "libraries/{library_id}/book-list"
  delete_book => DELETE "libraries/{library_id}/{book_id}"
});

lv_server::events!(BookListEvents {
  Reload "from:body"
});

impl api::get_library_book_list::Router {
  pub async fn endpoint(Need(library): Need<Library>) -> HttpResponse {
    let books = library.books().unwrap();
    let view = BookList::render(&library.id, &books);

    lv_server::responses::html(view)
  }
}

impl api::delete_book::Router {
  pub async fn endpoint(Need((library, book)): Need<(Library, Book)>) -> HttpResponse {
    if book.fk_library != library.id {
      let view = BookList::render_book(&library.id, &book);

      return lv_server::responses::html(html!(
        (view)
        div id="alerts" hx-swap-oob="true" {
          "library / book mismatch"
        }
      ));
    }

    book.delete().unwrap();
    lv_server::responses::empty_html()
  }
}

impl lv_server::Fragment<BookListEvents, api::Router> for BookList {
  const ID: &'static str = "BookList";
}

impl BookList {
  pub fn render(library_id: &String, books: &Vec<Book>) -> Markup {
    html!(
      div.books
        hx-get={(api::get_library_book_list::url(&library_id))}
        hx-trigger={(BookListEvents::Reload)}
        hx-swap="outerHTML"
        hx-target="this" {

        ul
          // for the deletion
          hx-swap="outerHTML"
          hx-target="closest li" {
          @for book in books {
            (Self::render_book(library_id, book))
          }
        }

      }
    )
  }

  pub fn render_book(library_id: &str, book: &Book) -> Markup {
    html!(
      li {
        a href={(super::super::api::get_with_book::url(library_id, &book.id))} {(book.title)}
        button
          hx-confirm={"Delete book "(book.title)"?"}
          hx-delete={(api::delete_book::url(library_id, &book.id))}
          {"delete"}
      }
    )
  }
}
