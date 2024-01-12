use crate::prelude::*;

pub struct BookViewEditToggle;

impl lv_server::Fragment<(), api::Router> for BookViewEditToggle {
  const ID: &'static str = "BookViewToggle";
}
lv_server::endpoints!(BookViewEditToggle {
  get_index => GET "{library_id}/{book_id}"
  get_edit_form => GET "{library_id}/{book_id}/edit"
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

impl api::put_library_book::Router {
  pub async fn endpoint(Need(mut book): Need<Book>, Form(form): Form<Book>) -> HttpResponse {
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
      div.document hx-swap="outerHTML" hx-target="this" {
        h1 {(book.title)}
        div.actions {
          button hx-get={(api::get_edit_form::url(&book.fk_library, &book.id))} {"Edit book"}
        }

        pre {(book.content)}
      }
    )
  }

  pub fn render_edit_form(book: &Book) -> Markup {
    html!(
      div.document hx-swap="outerHTML" hx-target="this" {
        form hx-put={(api::put_library_book::url(&book.fk_library, &book.id))} {
          div {
            button {"save"}
            button hx-get={(api::get_index::url(&book.fk_library, &book.id))} {"cancel"}
          }
          input name="title" value={(book.title)};
          textarea name="content" {(book.content)}
        }
      }
    )
  }
}
