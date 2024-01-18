use crate::prelude::*;

pub struct BookViewEditToggle;

#[derive(Deserialize)]
pub struct UpdateBookForm {
  title: String,
  content: String
}

impl lv_server::Fragment<(), api::Router> for BookViewEditToggle {
  const ID: &'static str = "BookViewToggle";
}
lv_server::endpoints!(BookViewEditToggle {
  get_index => GET "{library_id}/{book_id}"
  get_edit_form => GET "{library_id}/{book_id}/edit"
  get_actions => GET "{library_id}/{book_id}/actions"
  post_like => POST "{book_id}/like"
  post_unlike => POST "{book_id}/unlike"
  put_library_book => PUT "{library_id}/{book_id}"
});

impl api::get_index::Router {
  pub async fn endpoint(Need(book): Need<Book>) -> HttpResponse {
    lv_server::responses::html(BookViewEditToggle::render(&book))
  }
}

impl api::get_edit_form::Router {
  pub async fn endpoint(Need(book): Need<Book>) -> HttpResponse {
    lv_server::responses::html(BookViewEditToggle::render_edit_form(&book))
  }
}

impl api::get_actions::Router {
  pub async fn endpoint(Need(book): Need<Book>) -> HttpResponse {
    let view = BookViewEditToggle::render_actions(&book).await;

    lv_server::responses::html(view)
  }
}

impl api::post_like::Router {
  pub async fn endpoint(Need(book): Need<Book>) -> TemplateResponse {
    let user = dev::signed_user();

    let some_like = LikedBook::find_by_author_and_book(&user.id, &book.id)
      .await
      .internal_error_if_err("unable to verify if book is already liked")?;

    if some_like.is_none() {
      let _ = LikedBook::default()
        .add(user.id, book.id.clone())
        .internal_error_if_err("failed to create like record")?;
    }

    Ok(BookViewEditToggle::render_like_buttons(&book.id, true))
  }
}

impl api::post_unlike::Router {
  pub async fn endpoint(Need(book): Need<Book>) -> TemplateResponse {
    let user = dev::signed_user();

    let some_like = LikedBook::find_by_author_and_book(&user.id, &book.id)
      .await
      .internal_error_if_err("unable to verify if book is already liked")?;

    if let Some(like) = some_like {
      like
        .delete()
        .await
        .internal_error_if_err("failed to delete like record")?;
    }

    Ok(BookViewEditToggle::render_like_buttons(&book.id, false))
  }
}

impl api::put_library_book::Router {
  pub async fn endpoint(
    Need(mut book): Need<Book>, Form(form): Form<UpdateBookForm>
  ) -> HttpResponse {
    book.content = form.content;
    book.title = form.title;
    book.update().unwrap();

    let view = BookViewEditToggle::render(&book);
    let html = lv_server::responses::html(view);

    super::BookListEvents::Reload.trigger(html)
  }
}

impl BookViewEditToggle {
  pub fn render(book: &Book) -> Markup {
    html!(
      div.document {
        div.actions
          hx-get={(api::get_actions::url(&book.fk_library, &book.id))}
          hx-trigger="load" {}

        (book)
      }
    )
  }

  pub(self) async fn render_actions(book: &Book) -> Markup {
    let user = dev::signed_user();
    let Ok(is_author) = book.is_author(&user.id).await else {
      return html!(
        div.actions {
          "failed to fetch personalised actions"
        }
      );
    };

    let likes = LikedBook::does_like_book(&user.id, &book.id)
      .await
      .unwrap_or_default();

    html!(
      div.actions {
        @if is_author {
          button
            hx-get={(api::get_edit_form::url(&book.fk_library, &book.id))}
            hx-swap="outerHTML"
            hx-target="closest .document"
            {"Edit book"}
        }

        (Self::render_like_buttons(&book.id, likes))
      }
    )
  }

  fn render_like_buttons(book: &str, like: bool) -> Markup {
    match like {
      true => html!(
        button
          hx-post={(api::post_unlike::url(book))}
          {"Unlike"}
      ),
      false => html!(
        button
          hx-post={(api::post_like::url(book))}
          {"Like"}
      )
    }
  }

  pub fn render_edit_form(book: &Book) -> Markup {
    html!(
      div.document
        hx-swap="outerHTML"
        hx-target="this"
      {
        form hx-put={(api::put_library_book::url(&book.fk_library, &book.id))} {
          input name="title" value={(book.title)};
          textarea class="mtop" name="content" {(book.content)}

          div.mtop {
            button {"save"}
            button
              hx-get={(api::get_index::url(&book.fk_library, &book.id))}
              {"cancel"}
          }
        }
      }
    )
  }
}
