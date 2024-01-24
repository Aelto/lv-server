use crate::prelude::*;

mod crud;
mod domain;

pub mod partials;
pub use partials::LibraryRecommendationsParams;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LibraryRecommendations {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub library: ForeignKey<Library, Id>,
  pub approved: ForeignKey<Vec<Book>, Vec<Id>>,
  pub pending: ForeignKey<Vec<Book>, Vec<Id>>,
  pub denied: ForeignKey<Vec<Book>, Vec<Id>>
}

surreal_simple_querybuilder::model!(LibraryRecommendations {
  id,
  pub library,
  pub approved,
  pub pending,
  pub denied
});
pub use schema::model;
crate::with_model!(LibraryRecommendations);

impl IntoKey<Id> for LibraryRecommendations {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}

impl LibraryRecommendations {
  pub fn custom_id(library: &Id) -> Id {
    Id::new_thing(model.to_string(), library.id())
  }
}
