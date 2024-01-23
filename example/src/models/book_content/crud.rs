use crate::prelude::*;

impl BookContent {
  pub async fn create(mut self, book: &Id) -> AppResult<Self> {
    self.id = Self::custom_id(&book);
    Model::m_create(self).await
  }

  pub async fn find_by_id(id: &Id) -> AppResult<Option<Self>> {
    Self::m_find_one(id, ()).await
  }

  pub async fn find_by_book_id(book_id: &Id) -> AppResult<Option<Self>> {
    Self::find_by_id(&Self::custom_id(book_id)).await
  }

  pub async fn update(self) -> AppResult<Self> {
    self.m_update().await
  }
}
