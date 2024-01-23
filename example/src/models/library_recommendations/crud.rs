use crate::prelude::*;

use super::LibraryRecommendationsParams;

impl LibraryRecommendations {
  pub async fn create(mut self, library: Id) -> AppResult<Self> {
    if self.approved.is_unloaded() {
      self.approved.set_key(vec![]);
    }

    if self.to_approve.is_unloaded() {
      self.to_approve.set_key(vec![]);
    }

    if self.denied.is_unloaded() {
      self.denied.set_key(vec![]);
    }

    self.id = Self::custom_id(&library);
    self.library.set_key(library);

    Model::m_create(self).await
  }

  pub async fn find_all() -> AppResult<Vec<Self>> {
    Self::m_find(()).await
  }

  pub async fn find_by_id(
    id: &Id, params: LibraryRecommendationsParams
  ) -> AppResult<Option<Self>> {
    Self::m_find_one(id, params).await
  }

  pub async fn find_by_library_id(
    library: &Id, params: LibraryRecommendationsParams
  ) -> AppResult<Option<Self>> {
    let id = Self::custom_id(library);

    Self::find_by_id(&id, params).await
  }

  pub async fn find_or_create(
    library: &Id, params: LibraryRecommendationsParams
  ) -> AppResult<Self> {
    match Self::find_by_library_id(library, params).await? {
      Some(l) => Ok(l),
      None => Self::default().create(library.to_owned()).await
    }
  }

  pub async fn update(self) -> AppResult<Self> {
    self.m_update().await
  }

  pub async fn delete(self) -> AppResult<()> {
    self.m_delete().await?;

    Ok(())
  }
}
