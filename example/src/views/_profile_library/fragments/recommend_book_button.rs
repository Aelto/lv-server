use crate::prelude::*;

pub struct RecommendBookButton;

impl lv_server::Fragment<(), api::Router> for RecommendBookButton {
  const ID: &'static str = "RecommendBookButton";
}

lv_server::endpoints!(RecommendBookButton {
  get_recommend_book_button => GET "{library_id}/button"
  get_recommend_book_form => GET "{library_id}/form"
  post_book_recommendation => POST "{library_id}"
});

impl api::get_recommend_book_button::Router {
  pub async fn endpoint(path: Path<Id>) -> HttpResponse {
    let id = path.into_inner();

    lv_server::responses::html(RecommendBookButton::render(&id))
  }
}

impl api::get_recommend_book_form::Router {
  pub async fn endpoint(Need(library): Need<Library>) -> AppResponse {
    let likes = LikedBook::find_by_author(dev::signed_user().await.id_res()?).await?;
    let books = LikedBook::as_books(&likes).await?;

    let body = RecommendBookButton::render_form(&library, &books);
    let res = lv_server::responses::html(body);

    Ok(res)
  }
}

impl api::post_book_recommendation::Router {
  pub(self) async fn endpoint(
    Need(library): Need<Library>, Form(data): Form<RecommendBookForm>
  ) -> AppResponse {
    let fragment = RecommendBookButton::render(library.id_res()?);

    LibraryRecommendations::add_to_approve_recommendation().await;

    let res = lv_server::responses::html(fragment);
    let res = super::BookListRecommendationsEvents::Reload.trigger(res);

    Ok(res)
  }
}

#[derive(Serialize, Deserialize)]
struct RecommendBookForm {
  book_id: String
}

impl RecommendBookButton {
  pub fn render(library_id: &Id) -> Markup {
    html!(
      button
        hx-target="this"
        hx-get={(api::get_recommend_book_form::url(library_id.id()))}
        {"Recommend book"}
    )
  }

  fn render_form(lib: &Library, books: &Vec<Book>) -> Markup {
    html!(
      form
        hx-post={(api::post_book_recommendation::url(&lib.id()))}
        hx-target="this" {

        h2 {"Recommend a book in "(lib.title)}

        div {
          label {"Recently liked book"}
          @if books.is_empty() {
            div {"You must like a book before recommending it"}
          }
          @else {
            select name="book_id" {
              @for book in books {
                option value={(book.id())} {(book.title)}
              }
            }
          }
        }

        div {
          button {"Recommend book"}
          button
            hx-get={(api::get_recommend_book_button::url(lib.id()))} {"Cancel"}
        }
      }
    )
  }
}
