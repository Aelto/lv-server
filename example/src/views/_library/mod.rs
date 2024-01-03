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
      lv_server::responses::html(page(params.render(&lib)))
    }
  }
}

impl ViewLibrary {
  fn render(&self, lib: &Library) -> Markup {
    html!(
      header {
        h1 {"Library: "(lib.title)}
      }

      div.library {
        @if let Ok(books) = lib.books() {
          div.sidebar {
            (fragments::AddBookButton::render(&lib.id))
            hr;
            (fragments::BookList::render(&lib.id, &books))
          }
        }

        @if let Some(book_id) = &self.book {
          @if let Ok(Some(book)) = lib.book(&book_id) {
            (fragments::BookViewEditToggle::render(&book))
          }
        }
      }
    )
  }
}
