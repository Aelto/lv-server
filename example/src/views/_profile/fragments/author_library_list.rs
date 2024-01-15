use crate::prelude::*;

pub struct AuthorLibraryList;

lv_server::endpoints!(AuthorLibraryList {
  get_author_library_list => GET "profile/{author_id}/libraries"
});

lv_server::events!(AuthorLibraryListEvents {
  Reload "from:body"
});

impl api::get_author_library_list::Router {
  async fn endpoint(Need(author): Need<Author>) -> HttpResponse {
    let libraries = author.libraries().unwrap();

    lv_server::responses::html(AuthorLibraryList::render(&author, &libraries))
  }
}

impl lv_server::Fragment<AuthorLibraryListEvents, api::Router> for AuthorLibraryList {
  const ID: &'static str = "AuthorLibraryList";
}

impl AuthorLibraryList {
  pub fn render(author: &Author, libraries: &Vec<Library>) -> Markup {
    html!(
      ul class="libraries"
        hx-get={(api::get_author_library_list::url(&author.id))}
        hx-trigger={(AuthorLibraryListEvents::Reload)}
        hx-swap="outerHTML"
        hx-target="this"
        {
          @for library in libraries {
            li {
              a href={(crate::views::_profile_library::api::get_index::url(&library.id))} {(library.title)}
            }
          }
      }
    )
  }
}
