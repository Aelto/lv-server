use crate::prelude::*;

mod crud;

pub mod partials;
pub use partials::LibraryRecommendationsParams;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LibraryRecommendations {
  #[serde(skip_serializing_if = "Id::is_empty")]
  pub id: Id,

  pub library: ForeignKey<Library, Id>,
  pub approved: ForeignKey<Vec<Book>, Vec<Id>>,
  pub to_approve: ForeignKey<Vec<Book>, Vec<Id>>
}

surreal_simple_querybuilder::model!(LibraryRecommendations {
  id,
  pub library,
  pub approved,
  pub to_approve
});
pub use schema::model;
crate::with_model!(LibraryRecommendations);

impl IntoKey<Id> for LibraryRecommendations {
  fn into_key(&self) -> Result<Id, IntoKeyError> {
    Ok(self.id.clone())
  }
}

impl LibraryRecommendations {
  pub async fn add_to_approve_recommendation() {
    todo!()
  }

  pub async fn approve_recommendation() {
    todo!()
  }
}
