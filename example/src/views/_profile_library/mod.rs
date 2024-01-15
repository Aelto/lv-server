use crate::prelude::*;

pub mod fragments;

#[derive(Debug, Deserialize)]
pub struct ViewProfileLibrary;

impl
  lv_server::View<(
    fragments::AddBookButton,
    fragments::BookList,
    fragments::BookViewEditToggle
  )> for ViewProfileLibrary
{
}

lv_server::endpoints!(ViewProfileLibrary as view {
  get_index => GET "/library/{library_id}"
  get_with_book => GET "/library/{library_id}/{book_id}"
});

impl api::get_index::Router {
  async fn endpoint(Need(lib): Need<Library>) -> HttpResponse {
    lv_server::responses::html(page(ViewProfileLibrary::render(&lib, None)))
  }
}

impl api::get_with_book::Router {
  async fn endpoint(Need((lib, book)): Need<(Library, Book)>) -> HttpResponse {
    lv_server::responses::html(page(ViewProfileLibrary::render(&lib, Some(&book))))
  }
}

impl ViewProfileLibrary {
  fn render(lib: &Library, book: Option<&Book>) -> Markup {
    html!(
      header {
        h1 {"Library: "(lib.title)}
      }

      div.library {
        @if let Ok(books) = lib.books() {
          div.sidebar {
            (fragments::AddBookButton::render(&lib.id))
            hr;
            (fragments::BookList::render(&lib.id, &books))
          }
        }

        @if let Some(book) =  book {
          (fragments::BookViewEditToggle::render(&book))
        }
      }
    )
  }
}
