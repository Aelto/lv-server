use actix_web::error::ErrorUnauthorized;

use crate::prelude::*;

pub struct BookListRecommendations;

impl lv_server::Fragment<BookListRecommendationsEvents, api::Router> for BookListRecommendations {
  const ID: &'static str = "BookListRecommendations";
}

lv_server::endpoints!(BookListRecommendations {
  get_index => GET "{library_id}"
  put_approve_book => PUT "{library_id}/{recommended_book_id}"
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

impl api::put_approve_book::Router {
  pub async fn endpoint(
    Need((library, mut recommandation)): Need<(Library, RecommendedBook)>
  ) -> AppResponse {
    if library.fk_author != dev::signed_user().id {
      return Ok(ErrorUnauthorized("insufficient_permissions").into());
    }

    recommandation.approved = true;
    recommandation.update().await?;

    Ok(BookListRecommendationsEvents::Reload.trigger(lv_server::responses::no_content()))
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

        @if !approved.is_empty() {
          (Self::render_approved_books(&approved))
        }

        @if is_author && !unapproved.is_empty() {
          (Self::render_unapproved_books(&unapproved))
        }
      }
    ))
  }

  fn render_approved_books(recommendations: &Vec<(RecommendedBook, Book)>) -> Markup {
    html!(
      div {"Recommended books"}
      ul {
        @for (_, book) in recommendations {
          li {
            a href={(super::super::api::get_with_book::url(&book.fk_library, &book.id))} {(book.title)}
          }
        }
      }
    )
  }

  fn render_unapproved_books(recommendations: &Vec<(RecommendedBook, Book)>) -> Markup {
    html!(
      div {"Recommended books (need approval)"}
        ul {
          @for (rec, book) in recommendations {
            li {
              a href={(super::super::api::get_with_book::url(&book.fk_library, &book.id))} {(book.title)}
              button
                hx-confirm={"Approve recommendation for "(book.title)"?"}
                hx-put={(api::put_approve_book::url(&book.fk_library, &rec.id))}
                hx-target="closest li"
                {"approve"}
            }
          }
        }
    )
  }
}
