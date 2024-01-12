use crate::prelude::*;

mod fragments;

#[derive(Debug)]
pub struct ViewProfile;

impl lv_server::View<(fragments::AddLibraryButton, fragments::AuthorLibraryList)> for ViewProfile {}

impl WithRouter for ViewProfile {
  fn router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/profile/{author_id}", get().to(index));

    async fn index(Need(PEAuthor(author)): Need<PEAuthor>) -> HttpResponse {
      lv_server::responses::html(page(ViewProfile::render(&author)))
    }
  }
}

impl ViewProfile {
  pub fn render(author: &Author) -> Markup {
    let libraries = author.libraries().unwrap();

    html!(
      h2 {(author.handle)}

      section {
        h3 {"Libraries"}
        (fragments::AuthorLibraryList::render(&author, &libraries))
        (fragments::AddLibraryButton::render_button(&author.id))
      }
    )
  }
}
