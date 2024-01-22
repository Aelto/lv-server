use crate::prelude::*;

use super::model;

impl Library {
  pub async fn create(self, author_id: &Id) -> AppResult<Self> {
    let lib = Model::m_create(self).await?;

    Author::add_library_record(author_id, &lib.id).await?;

    Ok(lib)
  }

  pub async fn find_all() -> AppResult<Vec<Self>> {
    Self::m_find(()).await
  }

  pub async fn find_by_id(id: &Id, params: LibraryParams) -> AppResult<Option<Self>> {
    Self::m_find_one(id, params).await
  }

  pub async fn find_by_title(title: &str, params: LibraryParams) -> AppResult<Option<Self>> {
    let filter = Where((model.title, title));
    Self::m_find((filter, params)).await
  }

  pub async fn find_by_author(author: &str) -> AppResult<Vec<Self>> {
    let filter = Where((model.author, author));
    Self::m_find(filter).await
  }

  pub async fn books(&self) -> AppResult<Vec<Book>> {
    let some: Option<Self> = Self::m_find_one(&self.id, LibraryParams::FetchBooks).await?;
    let self_with_books: Self = some.conflict_if_none("record_not_found")?;

    Ok(
      self_with_books
        .books
        .into_inner()
        .into_value()
        .unwrap_or_default()
    )
  }

  // pub async fn recommended_books(
  //   &self
  // ) -> AppResult<(Vec<(RecommendedBook, Book)>, Vec<(RecommendedBook, Book)>)> {
  //   todo!()

  // let recommendations = RecommendedBook::find_all_by_library(&self.id).await?;
  // let mut approved = Vec::with_capacity(recommendations.len());
  // let mut unapproved = Vec::new();

  // // if/when the find_by_id becomes async it might be a good idea to join the
  // // futures or to use a mpsc channel.
  // for rec in recommendations {
  //   let book = Book::find_by_id(&rec.fk_book)?;
  //   if let Some(book) = book {
  //     if rec.approved {
  //       approved.push((rec, book));
  //     } else {
  //       unapproved.push((rec, book));
  //     }
  //   }
  // }

  // Ok((approved, unapproved))
  // }
}
