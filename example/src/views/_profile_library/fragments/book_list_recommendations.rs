use crate::prelude::*;

pub struct BookListRecommendations;

impl lv_server::Fragment<BookListRecommendationsEvents, api::Router> for BookListRecommendations {
  const ID: &'static str = "BookListRecommendations";
}

lv_server::endpoints!(BookListRecommendations {
  get_index => GET "{library_id}"
});

lv_server::events!(BookListRecommendationsEvents {
  Reload "from:body"
});

impl api::get_index::Router {
  pub async fn endpoint(Need(library): Need<Library>) -> AppResponse {
    let books = library.recommended_books().await?;
    let view = BookListRecommendations::render(&library.id, &books);

    Ok(lv_server::responses::html(view))
  }
}

impl BookListRecommendations {
  pub fn render(library_id: &str, books: &Vec<Book>) -> Markup {
    html!(
      section class="recommended-books"
        hx-get={(api::get_index::url(library_id))}
        hx-trigger={(BookListRecommendationsEvents::Reload)}
        hx-target="this" {

        div {"Recommended books"}

        ul {
          @for book in books {
            (Self::render_book(book))
          }
        }
      }
    )
  }

  pub fn render_book(book: &Book) -> Markup {
    html!(
      li {
        a href={(super::super::api::get_with_book::url(&book.fk_library, &book.id))} {(book.title)}
      }
    )
  }
}
