use crate::prelude::*;

/// Displays the list of recently written articles
pub struct RecentArticleList;

lv_server::events!(RecentArticleListEvents {
  Reload "from:body"
});

lv_server::endpoints!(RecentArticleList {
  get_list => GET "articles"
});

impl api::get_list::Router {
  async fn endpoint() -> HttpResponse {
    let books = Book::find_most_recent(10).await.unwrap_or_default();

    lv_server::responses::html(RecentArticleList::render(&books))
  }
}

impl lv_server::Fragment<(), ()> for RecentArticleList {
  const ID: &'static str = "RecentArticleList";
}

impl RecentArticleList {
  pub fn render_load_fetcher() -> Markup {
    html!(
      div
        hx-get={(api::get_list::url())}
        hx-trigger="load"
        {}
    )
  }
  fn render(books: &Vec<Book>) -> Markup {
    html!(
      ul {
        @for book in books {
          li {a href={(crate::views::_profile_library::api::get_with_book::url(&book.fk_library, &book.id))} {(book.title)}}
        }
      }
    )
  }
}
