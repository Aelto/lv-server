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
  pub to_approve: ForeignKey<Vec<Book>, Vec<Id>>,
  pub denied: ForeignKey<Vec<Book>, Vec<Id>>
}

surreal_simple_querybuilder::model!(LibraryRecommendations {
  id,
  pub library,
  pub approved,
  pub to_approve,
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

  pub fn is_approved(&self, book: &Id) -> AppResult<bool> {
    Ok(
      self
        .approved
        .to_key()?
        .conflict_if_none("empty_approved_list")?
        .iter()
        .any(|id| book == id)
    )
  }

  pub fn is_denied(&self, book: &Id) -> AppResult<bool> {
    Ok(
      self
        .denied
        .to_key()?
        .conflict_if_none("empty_denied_list")?
        .iter()
        .any(|id| book == id)
    )
  }

  pub async fn add_to_approve_recommendation(mut self, book: Book) -> AppResult<Self> {
    if self.is_denied(&book.id)? || self.is_approved(&book.id)? {
      return Ok(self);
    }

    self.to_approve.push(book)?;
    self.update().await
  }

  pub async fn approve_recommendation_by_library(library: &Id, book_id: &Id) -> AppResult<Self> {
    let id = Self::custom_id(library);

    Self::approve_recommendation(&id, book_id).await
  }

  pub async fn approve_recommendation(recommandations_id: &Id, book_id: &Id) -> AppResult<Self> {
    let mut rec = Self::find_by_id(
      recommandations_id,
      LibraryRecommendationsParams::FetchToApprove
    )
    .await?
    .not_found_if_none("recommendations_not_found")?;

    if rec.is_denied(book_id)? || rec.is_approved(&book_id)? {
      return Ok(rec);
    }

    let Some(index) = rec
      .to_approve
      .value()
      .internal_error_if_none("recommendations_empty_to_approve")?
      .iter()
      .position(|book| &book.id == book_id)
    else {
      return Ok(rec);
    };

    let book = rec
      .to_approve
      .value_mut()
      .internal_error_if_none("recommendations_empty_to_approve")?
      .remove(index);

    rec.approved.push(book)?;
    rec.update().await
  }
}
