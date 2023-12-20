use crate::prelude::*;

pub struct BookList(pub Vec<Book>);

impl maud::Render for BookList {
  fn render(&self) -> Markup {
    html!(
      ul class="list-books" hx-get="/app/books" hx-trigger={"fetch-book-list from:body"} hx-swap="outerHTML" {
        @for b in &self.0 {
          li {(b)}
        }
      }
    )
  }
}

impl lv_server::WithRouter for BookList {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .route("/app/books", get().to(display_book_list))
      .route("/app/books", post().to(add_book));

    async fn display_book_list() -> HttpResponse {
      let books = BookList(Book::find_all().unwrap());

      lv_server::responses::as_html(&books)
    }

    async fn add_book(Form(book): Form<Book>) -> HttpResponse {
      book.add().unwrap();

      let fragment = display_book_list().await;
      lv_server::responses::trigger(fragment, "fetch-book-list")
    }
  }
}
