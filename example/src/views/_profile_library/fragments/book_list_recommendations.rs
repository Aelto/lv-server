use actix_web::error::ErrorUnauthorized;

use crate::prelude::*;

pub struct BookListRecommendations;

impl lv_server::Fragment<BookListRecommendationsEvents, api::Router> for BookListRecommendations {
  const ID: &'static str = "BookListRecommendations";
}

lv_server::endpoints!(BookListRecommendations {
  get_index => GET "{library_id}"
  put_approve_book => PUT "{library_id}/{book_id}"
});

lv_server::events!(BookListRecommendationsEvents {
  Reload "from:body"
});

impl api::get_index::Router {
  pub async fn endpoint(Need(library): Need<Library>) -> AppResponse {
    let view =
      BookListRecommendations::render(&library, library.is_author(&dev::signed_user().await)?);

    Ok(lv_server::responses::html(view.await.render()))
  }
}

impl api::put_approve_book::Router {
  pub async fn endpoint(Need((library, book)): Need<(Library, Book)>) -> AppResponse {
    if !library.is_author(&dev::signed_user().await)? {
      return Ok(ErrorUnauthorized("insufficient_permissions").into());
    }

    LibraryRecommendations::approve_recommendation_by_library(&library.id, &book.id).await?;

    Ok(BookListRecommendationsEvents::Reload.trigger(lv_server::responses::no_content()))
  }
}

impl BookListRecommendations {
  pub async fn render(library: &Library, is_author: bool) -> TemplateResponse {
    let Some(recs) = LibraryRecommendations::find_by_library_id(
      &library.id,
      LibraryRecommendationsParams::FetchFull
    )
    .await?
    else {
      return Ok(html!());
    };

    Ok(html!(
      section class="recommended-books"
        hx-trigger={(BookListRecommendationsEvents::Reload)}
        hx-get={(api::get_index::url(&library.id()))}
        hx-target="this" {

        @if let Some(approved) = recs.approved.value() {
          @if !approved.is_empty() {
            (Self::render_approved_books(&approved))
          }
        }

        @if let Some(unapproved) = recs.to_approve.value() {
          @if is_author && !unapproved.is_empty() {
            (Self::render_unapproved_books(&unapproved))
          }
        }
      }
    ))
  }

  fn render_approved_books(recommendations: &Vec<Book>) -> Markup {
    html!(
      div {"Recommended books"}
      ul {
        @for book in recommendations {
          li {
            a href={(super::super::api::get_with_book::url(book.library.fk().id(), book.id()))} {(book.title)}
          }
        }
      }
    )
  }

  fn render_unapproved_books(recommendations: &Vec<Book>) -> Markup {
    html!(
      div {"Recommended books (need approval)"}
        ul {
          @for book in recommendations {
            li {
              a href={(super::super::api::get_with_book::url(book.library.fk().id(), book.id()))} {(book.title)}
              button
                hx-confirm={"Approve recommendation for "(book.title)"?"}
                hx-put={(api::put_approve_book::url(book.library.fk().id(), book.id()))}
                hx-target="closest li"
                {"approve"}
            }
          }
        }
    )
  }
}
