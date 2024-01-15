use crate::prelude::*;

mod fragments;

#[derive(Debug)]
pub struct ViewProfile;

impl lv_server::View<(fragments::AddLibraryButton, fragments::AuthorLibraryList)> for ViewProfile {}

lv_server::endpoints!(ViewProfile as view {
  get_index => GET "/profile/{author_id}"
});

impl api::get_index::Router {
  async fn endpoint(Need(author): Need<Author>) -> HttpResponse {
    lv_server::responses::html(page(ViewProfile::render(&author)))
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
