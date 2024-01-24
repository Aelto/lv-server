use actix_web::error::ErrorUnauthorized;

use crate::prelude::*;

pub struct BookListRecommendations;

impl lv_server::Fragment<BookListRecommendationsEvents, api::Router> for BookListRecommendations {
  const ID: &'static str = "BookListRecommendations";
}

lv_server::endpoints!(BookListRecommendations {
  get_index => GET "{library_id}"
  put_approve_book => PUT "{library_id}/{book_id}"
  put_deny_book => PUT "{library_id}/{book_id}/deny"
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

    LibraryRecommendations::add_approved_recommendation(&library, book).await?;

    Ok(BookListRecommendationsEvents::Reload.trigger(lv_server::responses::no_content()))
  }
}

impl api::put_deny_book::Router {
  pub async fn endpoint(Need((library, book)): Need<(Library, Book)>) -> AppResponse {
    if !library.is_author(&dev::signed_user().await)? {
      return Ok(ErrorUnauthorized("insufficient_permissions").into());
    }

    LibraryRecommendations::add_denied_recommendation(&library, book).await?;

    Ok(BookListRecommendationsEvents::Reload.trigger(lv_server::responses::no_content()))
  }
}

impl BookListRecommendations {
  pub async fn render(library: &Library, is_author: bool) -> TemplateResponse {
    let recs = LibraryRecommendations::find_by_library_id(
      &library.id,
      LibraryRecommendationsParams::FetchFull
    )
    .await?
    .unwrap_or_default();

    Ok(html!(
      section class="recommended-books"
        hx-trigger={(BookListRecommendationsEvents::Reload)}
        hx-get={(api::get_index::url(&library.id()))}
        hx-target="this" {

        @if is_author {
          @if let Some(approved) = recs.approved.value() {
            @if !approved.is_empty() {
              (Self::render_approved_books_for_author(&approved))
            }
          }

          @if let Some(unapproved) = recs.pending.value() {
            @if !unapproved.is_empty() {
              (Self::render_pending_books(&unapproved))
            }
          }

          @if let Some(denied) = recs.denied.value() {
            @if !denied.is_empty() {
              (Self::render_denied_books(&denied))
            }
          }
        }
        @else {
          @if let Some(approved) = recs.approved.value() {
            @if !approved.is_empty() {
              (Self::render_approved_books(&approved))
            }
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

  fn render_approved_books_for_author(recommendations: &Vec<Book>) -> Markup {
    html!(
      div {"Recommended books"}
      ul {
        @for book in recommendations {
          li {
            a href={(super::super::api::get_with_book::url(book.library.fk().id(), book.id()))} {(book.title)}

            (Self::render_deny_button(book))
          }
        }
      }
    )
  }

  fn render_pending_books(recommendations: &Vec<Book>) -> Markup {
    html!(
      div {"Recommended books (need approval)"}
      ul {
        @for book in recommendations {
          li {
            a href={(super::super::api::get_with_book::url(book.library.fk().id(), book.id()))} {(book.title)}

            (Self::render_deny_button(book))
            (Self::render_approve_button(book))
          }
        }
      }
    )
  }

  fn render_denied_books(recommendations: &Vec<Book>) -> Markup {
    html!(
      div {"Denied books"}
      ul {
        @for book in recommendations {
          li {
            a href={(super::super::api::get_with_book::url(book.library.fk().id(), book.id()))} {(book.title)}

            (Self::render_approve_button(book))
          }
        }
      }
    )
  }

  fn render_approve_button(book: &Book) -> Markup {
    html!(
      button
        hx-confirm={"Approve recommendations for "(book.title)"?"}
        hx-put={(api::put_approve_book::url(book.library.fk().id(), book.id()))}
        hx-target="closest li"
        {"approve"}
    )
  }

  fn render_deny_button(book: &Book) -> Markup {
    html!(
      button
        hx-confirm={"Deny future recommendations for "(book.title)"?"}
        hx-put={(api::put_deny_book::url(book.library.fk().id(), book.id()))}
        hx-target="closest li"
        {"deny"}
    )
  }
}
