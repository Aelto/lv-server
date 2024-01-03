use crate::prelude::*;

pub mod fragments;

#[derive(Debug, Deserialize)]
pub struct ViewLibrary {
  book: Option<String>
}

impl
  lv_server::View<(
    fragments::AddBookButton,
    fragments::BookList,
    fragments::BookViewEditToggle
  )> for ViewLibrary
{
  fn identifier() -> &'static str {
    "ViewLibrary"
  }
}
impl lv_server::WithRouter for ViewLibrary {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/libraries/{library_id}", get().to(index));

    async fn index(
      Need(LibraryPathExt(lib)): Need<LibraryPathExt>, params: Query<ViewLibrary>
    ) -> HttpResponse {
      lv_server::responses::html(page(html!(
        header {
          h1 {"Library: "(lib.title)}
        }

        div.library {
          (render_sidebar(&lib).await)

          @if let Some(book_id) = &params.book {
            (render_document(&lib, book_id).await)
          }
        }
      )))
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

async fn render_document(lib: &Library, book_id: &str) -> Markup {
  let Some(book) = lib.book(&book_id).unwrap() else {
    return html!(div.document {
      "No document with this ID"
    });
  };

  fragments::BookViewEditToggle::render(&book)
}
