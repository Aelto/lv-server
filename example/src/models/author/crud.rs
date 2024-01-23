use crate::prelude::*;

impl Author {
  pub async fn create(mut self) -> AppResult<Self> {
    if self.libraries.is_unloaded() {
      self.libraries.set_key(vec![]);
    }

    Model::m_create(self).await
  }

  pub async fn find_all() -> AppResult<Vec<Self>> {
    Self::m_find(()).await
  }

  pub async fn find_by_id(id: &Id, params: AuthorParams) -> AppResult<Option<Self>> {
    Self::m_find_one(&id, params).await
  }

  pub async fn fetch_libraries(&self) -> AppResult<Vec<Library>> {
    let author = Self::find_by_id(&self.id, AuthorParams::FetchLibraries).await?;

    Ok(
      author
        .and_then(|a| a.libraries.into_inner().into_value())
        .unwrap_or_default()
    )
  }
}
