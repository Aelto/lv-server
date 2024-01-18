use crate::prelude::*;

pub mod fragments;

#[derive(Debug, Deserialize)]
pub struct ViewProfileLibrary;

impl
  lv_server::View<(
    (fragments::AddBookButton, fragments::RecommendBookButton),
    (fragments::BookList, fragments::BookListRecommendations),
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
    let view = ViewProfileLibrary::render(&lib, None).await;

    lv_server::responses::html(page(view.render()))
  }
}

impl api::get_with_book::Router {
  async fn endpoint(Need((lib, book)): Need<(Library, Book)>) -> HttpResponse {
    let view = ViewProfileLibrary::render(&lib, Some(&book)).await;

    lv_server::responses::html(page(view.render()))
  }
}

impl ViewProfileLibrary {
  async fn render(lib: &Library, book: Option<&Book>) -> TemplateResponse {
    Ok(html!(
      h1 {"Library: "(lib.title)}

      div.library {
        @if let Ok(books) = lib.books() {
          div.sidebar {
            (fragments::AddBookButton::render(&lib.id))
            hr;
            (fragments::BookList::render(&lib.id, &books))
            hr;
            @let recommendations = lib.recommended_books().await?;

            (fragments::RecommendBookButton::render(&lib.id))
            (fragments::BookListRecommendations::render(&lib.id, &recommendations))
          }
        }

        @if let Some(book) =  book {
          (fragments::BookViewEditToggle::render(&book))
        }
      }
    ))
  }
}
