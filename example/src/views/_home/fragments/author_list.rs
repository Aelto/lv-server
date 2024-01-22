use crate::prelude::*;

pub struct AuthorList;

lv_server::endpoints!(AuthorList {
  get_author_list => GET "authors"
});

impl api::get_author_list::Router {
  async fn endpoint() -> HttpResponse {
    let authors = Author::find_all().await.unwrap();

    lv_server::responses::html(AuthorList::render(&authors))
  }
}

lv_server::events!(AuthorListEvents {
  Reload "from:body"
});

impl lv_server::Fragment<AuthorListEvents, api::Router> for AuthorList {
  const ID: &'static str = "AuthorList";
}

impl AuthorList {
  pub fn render(authors: &Vec<Author>) -> Markup {
    html!(
      ul {
        @for author in authors {
          li {
            a href={(crate::views::_profile::api::get_index::url(author.id()))} {(author.handle)}
          }
        }
      }
    )
  }
}
