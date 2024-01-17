use crate::prelude::*;

pub struct BookListRecommandations;

lv_server::endpoints!(BookListRecommandations {
  get_index => GET "{library_id}"
});

lv_server::events!(BookListRecommandationsEvents {
  Reload "from:body"
});

impl lv_server::Fragment<BookListRecommandationsEvents, api::Router> for BookListRecommandations {
  const ID: &'static str = "BookListRecommandations";
}

impl api::get_index::Router {
  pub async fn endpoint(Need(library): Need<Library>) -> AppResponse {
    let books = library.recommended_books().await?;
    let view = BookListRecommandations::render(&books);

    Ok(lv_server::responses::html(view))
  }
}

impl BookListRecommandations {
  pub fn render(books: &Vec<Book>) -> Markup {
    html!(
      section class="recommended-books" {
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
