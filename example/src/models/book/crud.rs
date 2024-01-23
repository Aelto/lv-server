use crate::prelude::*;

impl Book {
  pub async fn create(mut self, library_id: &Id) -> AppResult<Self> {
    self.library.set_key(library_id.to_owned());

    let book = Model::m_create(self).await?;
    Library::add_book_record(library_id, &book.id).await?;

    Ok(book)
  }

  pub async fn find_all() -> AppResult<Vec<Self>> {
    Self::m_find(()).await
  }

  pub async fn find_by_id(id: &Id, params: BookParams) -> AppResult<Option<Self>> {
    Self::m_find_one(&id, params).await
  }

  pub async fn find_most_recent(page_size: usize) -> AppResult<Vec<Book>> {
    let mut all = Self::find_all().await?;
    all.sort_by(|a, b| {
      a.created_at
        .partial_cmp(&b.created_at)
        .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(all.into_iter().take(page_size).collect())
  }

  pub async fn update(self) -> AppResult<Self> {
    self.m_update().await
  }

  pub async fn delete(self) -> AppResult<()> {
    self.m_delete().await?;

    Ok(())
  }
}
