use crate::prelude::*;

impl BookRecommendation {
  pub async fn set_recommendation(
    library: &Library, book: &Book, status: LibraryRecommendationStatus
  ) -> AppResult<bool> {
    match Self::find_by_library_and_book(library, &book).await? {
      Some(mut rec) => {
        // a record for this book is already present, so it's either
        // approved/denied or pending. In all three cases a new pending record
        // shouldn't be added so better leave now
        if status.eq(&LibraryRecommendationStatus::Pending) {
          return Ok(false);
        }

        // no need to change anything, it's the same status
        if rec.status.eq(&status) {
          return Ok(false);
        }

        rec.status = status;
        rec.update().await?;
        Ok(true)
      }
      None => {
        Self::create(library, &book, status).await?;
        Ok(true)
      }
    }
  }
}
