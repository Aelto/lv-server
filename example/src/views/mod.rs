use crate::prelude::*;

mod library;

pub struct ViewIndex;
impl lv_server::View for ViewIndex {}

impl lv_server::WithRouter for ViewIndex {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
      .route("/", get().to(index))
      .route("/app/libraries", get().to(list_libraries))
      .route("/app/libraries", post().to(add_library));

    async fn index() -> HttpResponse {
      lv_server::responses::html(page(html!(
        h1{"Your libraries"}
        (render_libraries())
        hr;
        h2{"Create a library"}
        (render_library_form())
      )))
    }

    async fn list_libraries() -> HttpResponse {
      lv_server::responses::html(render_libraries())
    }

    async fn add_library(Form(lib): Form<Library>) -> HttpResponse {
      lib.add().unwrap();

      lv_server::responses::trigger(
        lv_server::responses::html(render_library_form()),
        "fetch-library-list"
      )
    }
  }
}

fn render_libraries() -> Markup {
  let libs = Library::find_all().unwrap();

  html!(
    ul class="list-libraries" hx-get="/app/libraries" hx-trigger={"fetch-library-list from:body"} hx-swap="outerHTML" {
      @for lib in libs {
        li {a href={"/library"(lib.id)} {(lib.title)}}
      }
    }
  )
}

fn render_library_form() -> Markup {
  html!(
    form hx-post="/app/libraries" {
      div {
        label for="title" {"Library title"}
        input name="title" type="text" value="lorem";
      }

      input type="submit";
    }
  )
}
