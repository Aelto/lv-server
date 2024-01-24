use crate::prelude::*;

use super::model;

impl BookRecommendation {
  pub async fn create(
    library: &Library, book: &Book, status: LibraryRecommendationStatus
  ) -> AppResult<Self> {
    let query = format!(
      "
      RELATE $book->{model}->$library
      SET {} = $id , {} = $status
    ",
      model.id, model.status
    );

    let mut response = DB
      .query(query)
      .bind(("library", library.id.to_thing()?))
      .bind(("id", Self::custom_id(book, library)))
      .bind(("book", book.id.to_thing()?))
      .bind(("status", status))
      .await?;

    let recommendation = response.take(0)?;
    unwrap_or_api_error(recommendation)
  }

  pub async fn find_by_library(library: &Library) -> AppResult<Vec<Self>> {
    // notes:
    // - we start from the library
    // - the "fetch in" gets us the book
    let query = format!(
      "
      SELECT *
      FROM $library<-{model}
      FETCH in
    "
    );

    let mut response = DB
      .query(query)
      .bind(("library", library.id.to_thing()?))
      .await?;

    let recommendations: Vec<Self> = response.take(0)?;

    Ok(recommendations)
  }

  pub async fn find_by_library_and_book(library: &Library, book: &Book) -> AppResult<Option<Self>> {
    Self::m_find_one(&Self::custom_id(book, library), ()).await
  }

  pub(super) async fn update(self) -> AppResult<Self> {
    self.m_update().await
  }

  pub async fn delete_if_exists(library: &Library, book: &Book) -> AppResult<()> {
    let recommendation = Self::find_by_library_and_book(library, book).await?;

    if let Some(rec) = recommendation {
      rec.delete().await?;
    }

    Ok(())
  }

  pub async fn delete(self) -> AppResult<()> {
    if let Some(id) = self.m_id() {
      let _: Option<Self> = DB.delete(id.to_thing()?).await?;
    }

    Ok(())
  }
}
