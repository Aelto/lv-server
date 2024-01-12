use crate::prelude::*;

pub mod fragments;

#[derive(Debug, Deserialize)]
pub struct ViewProfileLibrary {
  book: Option<String>
}

impl
  lv_server::View<(
    fragments::AddBookButton,
    fragments::BookList,
    fragments::BookViewEditToggle
  )> for ViewProfileLibrary
{
}

impl lv_server::WithRouter for ViewProfileLibrary {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/profile/{author_id}/{library_id}", get().to(index));

    async fn index(Need(lib): Need<Library>, params: Query<ViewProfileLibrary>) -> HttpResponse {
      lv_server::responses::html(page(params.render(&lib)))
    }
  }
}

impl ViewProfileLibrary {
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
