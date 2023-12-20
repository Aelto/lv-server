use crate::prelude::*;

pub struct LibraryCreateForm(Vec<Library>);

impl lv_server::Fragment for LibraryCreateForm {}

impl Render for LibraryCreateForm {
  fn render(&self) -> Markup {
    html!(
      ul class="list-libraries" hx-get="/app/libraries" hx-trigger={"fetch-library-list from:body"} hx-swap="outerHTML" {
        @for l in &self.0 {
          li {(l)}
        }
      }
    )
  }
}

impl lv_server::WithRouter for LibraryCreateForm {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .route("/app/libraries", get().to(display_library_list))
      .route("/app/libraries", post().to(add_library));

    async fn display_library_list() -> HttpResponse {
      let libraries = LibraryCreateForm(Library::find_all().unwrap());

      lv_server::responses::as_html(&libraries)
    }

    async fn add_library(Form(library): Form<Library>) -> HttpResponse {
      library.add().unwrap();

      let fragment = display_library_list().await;
      lv_server::responses::trigger(fragment, "fetch-library-list")
    }
  }
}
