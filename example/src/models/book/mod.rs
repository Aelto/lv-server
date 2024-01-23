use crate::prelude::*;

mod crud;
pub mod partials;
pub use partials::BookParams;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Book {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub title: String,
  pub library: ForeignKey<Library, Id>,

  // I've decided to not make it a field to avoid having to deal with the record
  // id, especially since the content is usually only loaded on demand and should
  // rarely be fetched using a `fetch` keyword
  //
  // pub content: ForeignKey<BookContent, Id>,
  pub created_at: chrono::DateTime<chrono::Utc>
}

surreal_simple_querybuilder::model!(Book {
  id,
  pub title,
  pub library,
  pub created_at
});
pub use schema::model;
crate::with_model!(Book);

impl Book {
  pub async fn is_author(&self, author_id: &Id) -> AppResult<bool> {
    let Some(lib) = Library::find_by_id(self.library.fk(), LibraryParams::None).await? else {
      return Ok(false);
    };

    Ok(lib.author.fk().eq(author_id))
  }

  pub async fn fetch_content(&self) -> AppResult<Option<BookContent>> {
    BookContent::find_by_book_id(&self.id).await
  }
}

impl IntoKey<Id> for Book {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}
