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
    let view =
      BookListRecommendations::render(&library, dev::signed_user().id == library.fk_author);

    Ok(lv_server::responses::html(view.await.render()))
  }
}

impl BookListRecommendations {
  pub async fn render(library: &Library, is_author: bool) -> TemplateResponse {
    let (approved, unapproved) = library.recommended_books().await?;

    Ok(html!(
      section class="recommended-books"
        hx-trigger={(BookListRecommendationsEvents::Reload)}
        hx-get={(api::get_index::url(&library.id))}
        hx-target="this" {

        (Self::render_approved_books(&approved))
        @if is_author {
          (Self::render_unapproved_books(&unapproved))
        }
      }
    ))
  }

  fn render_approved_books(books: &Vec<Book>) -> Markup {
    html!(
      div {"Recommended books"}
      ul {
        @for book in books {
          li {
            a href={(super::super::api::get_with_book::url(&book.fk_library, &book.id))} {(book.title)}
          }
        }
      }
    )
  }

  fn render_unapproved_books(books: &Vec<Book>) -> Markup {
    html!(
      div {"Recommended books (need approval)"}
        ul {
          @for book in books {
            (Self::render_book_need_approval(&book))
          }
        }
    )
  }

  fn render_book_need_approval(book: &Book) -> Markup {
    html!(
      li {
        a href={(super::super::api::get_with_book::url(&book.fk_library, &book.id))} {(book.title)}
      }
    )
  }
}
