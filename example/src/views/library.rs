use actix_web::web::Query;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct ViewLibrary {
  book: Option<String>
}

impl lv_server::View for ViewLibrary {}

impl lv_server::WithRouter for ViewLibrary {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .route("/libraries/{id}", get().to(index))
      .route(
        "/app/libraries/{id}/book-create-form-cancel",
        get().to(book_create_form_cancel)
      )
      .route(
        "/app/libraries/{id}/books/{book}",
        get().to(display_document)
      )
      .route(
        "/app/libraries/{id}/books/{book}/edit",
        get().to(display_book_edit_form)
      )
      .route(
        "/app/libraries/{id}/books/{book}",
        post().to(update_library_book)
      );

    async fn index(path: Path<String>, params: Query<ViewLibrary>) -> HttpResponse {
      let params = params.into_inner();
      let id = path.into_inner();

      let Some(lib) = Library::find_by_id(&id).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      lv_server::responses::html(page(html!(
        header {
          h1 {"Library: "(lib.title)}
        }

        div.library {
          (render_sidebar(&lib).await)

          @if let Some(book_id) = params.book {
            (render_document(&lib, book_id, false).await)
          }
        }
      )))
    }

    async fn book_create_form_cancel(path: Path<String>) -> HttpResponse {
      let id = path.into_inner();

      lv_server::responses::html(render_add_book_button(&id))
    }

    async fn display_document(path: Path<(String, String)>) -> HttpResponse {
      let (lib, doc) = path.into_inner();

      let Some(lib) = Library::find_by_id(&lib).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      lv_server::responses::html(render_document(&lib, doc, false).await)
    }

    async fn display_book_edit_form(path: Path<(String, String)>) -> HttpResponse {
      let (lib, doc) = path.into_inner();

      let Some(lib) = Library::find_by_id(&lib).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      lv_server::responses::html(render_document(&lib, doc, true).await)
    }

    async fn update_library_book(
      path: Path<(String, String)>, Form(form): Form<Book>
    ) -> HttpResponse {
      let (lib, doc) = path.into_inner();

      let Some(lib) = Library::find_by_id(&lib).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      let Some(mut book) = lib.book(&doc).unwrap() else {
        return lv_server::responses::as_html(&"No book with this ID");
      };

      book.content = form.content;
      book.title = form.title;
      dbg!(&book).update().unwrap();

      fragments::BookListEvents::Reload.trigger(lv_server::responses::html(
        render_document(&lib, doc, false).await
      ))
    }
  }
}

async fn render_sidebar(lib: &Library) -> Markup {
  let books = lib.books().unwrap();

  html!(div.sidebar {
    (fragments::AddBookButton::render(&lib.id))
    hr;
    (fragments::BookList::render(&lib.id, &books))
  })
}

async fn render_document(lib: &Library, book_id: String, edit: bool) -> Markup {
  let Some(book) = lib.book(&book_id).unwrap() else {
    return html!(div.document {
      "No document with this ID"
    });
  };

  html!(div.document hx-swap="outerHTML" hx-target="this" {
    @match edit {
      true => {
        form hx-post={"/app/libraries/"(lib.id)"/books/"(book_id)} {
          div {
            button {"save"}
            button hx-get={"/app/libraries/"(lib.id)"/books/"(book_id)} {"cancel"}
          }
          input name="title" value={(book.title)};
          textarea name="content" {(book.content)}
        }
      },
      false => {
        button hx-get={"/app/libraries/"(lib.id)"/books/"(book_id)"/edit"} {"edit"}

        pre {(book.content)}
      }
    }
  })
}

fn render_add_book_button(lib_id: &str) -> Markup {
  html!(
    button hx-target="this" hx-swap="outerHTML" hx-get={"/app/libraries/"(lib_id)"/book-create-form"} {"Add book"}
  )
}
