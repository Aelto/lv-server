use crate::prelude::*;

mod crud;

pub mod partials;
pub use partials::LikedBookParams;

/// A record optimized for fast O(1) querying when both ends of the link are
/// known. In this case ends are the author and the liked book.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LikedBook {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub author: ForeignKey<Author, Id>,
  pub book: ForeignKey<Book, Id>
}

surreal_simple_querybuilder::model!(LikedBook {
  id,
  pub author,
  pub book
});
pub use schema::model;
crate::with_model!(LikedBook);

impl LikedBook {
  pub async fn does_like_book(author: &Id, book: &Id) -> AppResult<bool> {
    Ok(Self::find_by_author_and_book(author, book).await?.is_some())
  }

  pub async fn get_liked_books(author: &Id) -> AppResult<Vec<Book>> {
    let likes = Self::find_by_author(author, LikedBookParams::FetchBook).await?;

    Ok(
      likes
        .into_iter()
        .filter_map(|l| l.book.into_inner().into_value())
        .collect()
    )
  }
}

impl LikedBook {
  pub fn custom_id(author_id: &Id, book_id: &Id) -> Id {
    let id = format!("{}/{}", author_id.id(), book_id.id());

    Id::new_thing(model.to_string(), id)
  }

  pub async fn book(&self) -> AppResult<Option<Book>> {
    Book::find_by_id(self.book.fk(), BookParams::None).await
  }
}

impl IntoKey<Id> for LikedBook {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}
