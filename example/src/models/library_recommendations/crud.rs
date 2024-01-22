use crate::prelude::*;

use super::{model, LibraryRecommendationsParams};

impl LibraryRecommendations {
  pub async fn create(self) -> AppResult<Self> {
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

  pub async fn find_all_by_library(library: &str) -> AppResult<Vec<Self>> {
    let filter = Where((model.library, library));
    let all = Self::m_find(filter).await?;

    Ok(all)
  }

  pub async fn update(self) -> AppResult<Self> {
    self.m_update().await
  }

  pub async fn delete(self) -> AppResult<()> {
    self.m_delete().await?;

    Ok(())
  }
}
