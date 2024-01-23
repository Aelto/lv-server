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
  pub async fn endpoint(Need(lib): Need<Library>) -> HttpResponse {
    lv_server::responses::html(RecommendBookButton::render(&lib.id))
  }
}

impl api::get_recommend_book_form::Router {
  pub async fn endpoint(Need(library): Need<Library>) -> AppResponse {
    let books = LikedBook::get_liked_books(&dev::signed_user().await.id).await?;
    let body = RecommendBookButton::render_form(&library, &books);
    let res = lv_server::responses::html(body);

    Ok(res)
  }
}

impl api::post_book_recommendation::Router {
  pub(self) async fn endpoint(
    Need(library): Need<Library>, Form(data): Form<RecommendBookForm>
  ) -> AppResponse {
    let book_id = Id::new_thing(crate::models::book::model.to_string(), data.book_id);
    let book = Book::find_by_id(&book_id, BookParams::None).await?;

    if let Some(book) = book {
      let mut recommandations =
        LibraryRecommendations::find_or_create(&library.id, LibraryRecommendationsParams::None)
          .await?;

      recommandations.to_approve.push(book)?;
      recommandations.update().await?;
    }

    let fragment = RecommendBookButton::render(&library.id);
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
          @if !books.is_empty() {
            button {"Recommend book"}
          }

          button
            hx-get={(api::get_recommend_book_button::url(lib.id()))} {"Cancel"}
        }
      }
    )
  }
}
