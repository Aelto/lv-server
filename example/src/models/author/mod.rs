use crate::prelude::*;

mod crud;
mod libraries;
pub mod partials;
pub use partials::AuthorParams;
pub const TABLE: &'static str = "Author";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Author {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub handle: String,
  pub libraries: ForeignKey<Vec<Library>, Vec<Id>>,
  pub created_at: chrono::DateTime<chrono::Utc>
}

surreal_simple_querybuilder::model!(Author {
  id,
  pub handle,
  pub libraries
});
pub use schema::model;
crate::with_model!(Author);

impl IntoKey<Id> for Author {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}
