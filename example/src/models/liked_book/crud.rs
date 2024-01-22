use crate::prelude::*;

use super::{model, LikedBookParams};

impl LikedBook {
  pub async fn create(mut self) -> AppResult<Self> {
    self.id = Self::custom_id(self.author.fk(), self.book.fk());

    Model::m_create(self).await
  }

  pub async fn find_all() -> AppResult<Vec<Self>> {
    Self::m_find(()).await
  }

  pub async fn find_by_id(id: &Id, params: LikedBookParams) -> AppResult<Option<Self>> {
    Self::m_find_one(&id, params).await
  }

  pub async fn find_by_author_and_book(author: &Id, book: &Id) -> AppResult<Option<Self>> {
    let id = Self::custom_id(author, book);

    Self::find_by_id(&id, LikedBookParams::None).await
  }

  pub async fn find_by_author(author: &Id, params: LikedBookParams) -> AppResult<Vec<Self>> {
    let filter = Where((model.author, author));

    Self::m_find((filter, params)).await
  }

  pub async fn update(self) -> AppResult<Self> {
    self.m_update().await
  }

  pub async fn delete(self) -> AppResult<()> {
    self.m_delete().await?;

    Ok(())
  }
}
