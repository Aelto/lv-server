use crate::prelude::*;

mod books;
mod crud;
pub mod partials;
pub use partials::LibraryParams;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Library {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub title: String,
  pub author: ForeignKey<Author, Id>,
  pub books: ForeignKey<Vec<Book>, Vec<Id>>,
  pub created_at: chrono::DateTime<chrono::Utc>
}

surreal_simple_querybuilder::model!(Library {
  id,
  pub title,
  pub author,
  pub books
});
pub use schema::model;
crate::with_model!(Library);

impl IntoKey<Id> for Library {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}

impl Library {
  pub fn is_author(&self, id: &Author) -> AppResult<bool> {
    Ok(self.author.to_key()?.unwrap_or_default().id() == id.id())
  }
}
