use crate::prelude::*;

pub struct ViewLibrary;
impl lv_server::View for ViewLibrary {}

impl lv_server::WithRouter for ViewLibrary {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .route("/libraries/{id}", get().to(index))
      .route(
        "/app/libraries/{id}/book-create-form",
        get().to(book_create_form)
      )
      .route(
        "/app/libraries/{id}/book-create-form-cancel",
        get().to(book_create_form_cancel)
      )
      .route("/app/libraries/{id}/books", post().to(add_library_book))
      .route("/app/libraries/{id}/books", get().to(list_books));

    async fn index(path: Path<String>) -> HttpResponse {
      let id = path.into_inner();

      let Some(lib) = Library::find_by_id(&id).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      lv_server::responses::html(page(html!(
        header {
          h1 {"Library: "(lib.title)}
        }

        (render_library(&lib).await)
      )))
    }

    async fn book_create_form(path: Path<String>) -> HttpResponse {
      let id = path.into_inner();

      let Some(lib) = Library::find_by_id(&id).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      lv_server::responses::html(render_book_create_form(&lib).await)
    }

    async fn book_create_form_cancel(path: Path<String>) -> HttpResponse {
      let id = path.into_inner();

      lv_server::responses::html(render_add_book_button(&id))
    }

    async fn add_library_book(path: Path<String>, Form(book): Form<Book>) -> HttpResponse {
      let id = path.into_inner();

      let Some(_) = Library::find_by_id(&id).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      let fragment = render_add_book_button(&id);
      book.add(id).unwrap();

      lv_server::responses::trigger(lv_server::responses::html(fragment), "fetch-book-list")
    }

    async fn list_books(path: Path<String>) -> HttpResponse {
      let id = path.into_inner();

      let Some(lib) = Library::find_by_id(&id).unwrap() else {
        return lv_server::responses::as_html(&"No library with this ID");
      };

      lv_server::responses::html(render_books(&lib).await)
    }
  }
}

async fn render_library(lib: &Library) -> Markup {
  html!(div.library {
    (render_sidebar(lib).await)
    (render_document(lib).await)
  })
}

async fn render_sidebar(lib: &Library) -> Markup {
  html!(div.sidebar {
    (render_add_book_button(&lib.id))
    (render_books(lib).await)
  })
}

async fn render_books(lib: &Library) -> Markup {
  let books = lib.books().unwrap();

  html!(
    div.books hx-get={"/app/libraries/"(lib.id)"/books"} hx-trigger={"fetch-book-list from:body"} {
      @for book in books {
        a href={"?"(book.id)} {(book.title)}
      }
    }
  )
}

async fn render_document(_: &Library) -> Markup {
  html!(div.document {

  })
}

fn render_add_book_button(lib_id: &str) -> Markup {
  html!(
    button hx-target="this" hx-swap="outerHTML" hx-get={"/app/libraries/"(lib_id)"/book-create-form"} {"Add book"}
  )
}

async fn render_book_create_form(lib: &Library) -> Markup {
  html!(
    form hx-post={"/app/libraries/"(lib.id)"/books"} hx-target="this" hx-swap="outerHTML" {

      h2 {"Add a book to "(lib.title)}

      div {
        label {"Title"}
        input type="text" name="title" placeholder="README";
      }

      button {"Create book"}
      button hx-get={"/app/libraries/"(lib.id)"/book-create-form-cancel"} {"Cancel"}
    }
  )
}
