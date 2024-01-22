use crate::prelude::*;

use super::model;

impl Library {
  pub async fn add_book_record(library_id: &Id, book_id: &Id) -> AppResult<()> {
    Self::m_add_one(library_id, &model.books, book_id).await?;

    Ok(())
  }
}
