use crate::prelude::*;

impl LibraryRecommendations {
  pub async fn add_pending_recommendation(library: &Library, book: Book) -> AppResult<Self> {
    let mut recs = fetch_for_edit_by_library(library).await?;

    if checks::is_pending(&recs, &book)?
      || checks::is_denied(&recs, &book)?
      || checks::is_approved(&recs, &book)?
    {
      return Ok(recs);
    }

    lists::set_in_pending(&mut recs, book)?;

    recs.update().await
  }

  pub async fn add_approved_recommendation(library: &Library, book: Book) -> AppResult<Self> {
    let mut recs = fetch_for_edit_by_library(library).await?;

    if checks::is_approved(&recs, &book)? {
      return Ok(recs);
    }

    lists::set_in_approved(&mut recs, book)?;

    recs.update().await
  }

  pub async fn add_denied_recommendation(library: &Library, book: Book) -> AppResult<Self> {
    let mut recs = fetch_for_edit_by_library(library).await?;

    if checks::is_denied(&recs, &book)? {
      return Ok(recs);
    }

    lists::set_in_denied(&mut recs, book)?;

    recs.update().await
  }
}

/// The ONLY way to get data before editing a DB record, it ensures all
/// necessary fields are fetched.
async fn fetch_for_edit_by_library(library: &Library) -> AppResult<LibraryRecommendations> {
  LibraryRecommendations::find_or_create(&library.id, LibraryRecommendationsParams::FetchFull).await
}

/// module for the functions that are used to check the current state of the
/// lists
mod checks {
  use crate::prelude::*;
  pub fn is_approved(recs: &LibraryRecommendations, book: &Book) -> AppResult<bool> {
    Ok(
      recs
        .approved
        .value()
        .conflict_if_none("empty_approved_list")?
        .iter()
        .any(|b| b.id == book.id)
    )
  }

  pub fn is_pending(recs: &LibraryRecommendations, book: &Book) -> AppResult<bool> {
    Ok(
      recs
        .denied
        .value()
        .conflict_if_none("empty_pendingd_list")?
        .iter()
        .any(|b| b.id == book.id)
    )
  }

  pub fn is_denied(recs: &LibraryRecommendations, book: &Book) -> AppResult<bool> {
    Ok(
      recs
        .denied
        .value()
        .conflict_if_none("empty_denied_list")?
        .iter()
        .any(|b| b.id == book.id)
    )
  }
}

/// module for the functions that mutate the lists while still ensuring workflow
/// rules (like a book can't be in two lists at the same time)
mod lists {
  use crate::prelude::*;

  pub fn set_in_approved(recs: &mut LibraryRecommendations, book: Book) -> AppResult<()> {
    remove_from_all_lists(recs, &book)?;
    recs.approved.push(book)?;

    Ok(())
  }

  pub fn set_in_denied(recs: &mut LibraryRecommendations, book: Book) -> AppResult<()> {
    remove_from_all_lists(recs, &book)?;
    recs.denied.push(book)?;

    Ok(())
  }

  pub fn set_in_pending(recs: &mut LibraryRecommendations, book: Book) -> AppResult<()> {
    remove_from_all_lists(recs, &book)?;
    recs.pending.push(book)?;

    Ok(())
  }

  fn remove_from_all_lists(recs: &mut LibraryRecommendations, book: &Book) -> AppResult<()> {
    remove_from_approved(recs, book)?;
    remove_from_denied(recs, book)?;
    remove_from_pending(recs, book)?;

    Ok(())
  }

  fn remove_from_approved(
    recs: &mut LibraryRecommendations, book: &Book
  ) -> AppResult<Option<Book>> {
    let Some(index) = recs
      .approved
      .value()
      .conflict_if_none("empty_approved_list")?
      .iter()
      .position(|b| b.id == book.id)
    else {
      return Ok(None);
    };

    let book = recs
      .approved
      .value_mut()
      .conflict_if_none("empty_approved_list")?
      .remove(index);

    Ok(Some(book))
  }

  fn remove_from_denied(recs: &mut LibraryRecommendations, book: &Book) -> AppResult<Option<Book>> {
    let Some(index) = recs
      .denied
      .value()
      .conflict_if_none("empty_denied_list")?
      .iter()
      .position(|b| b.id == book.id)
    else {
      return Ok(None);
    };

    let book = recs
      .denied
      .value_mut()
      .conflict_if_none("empty_denied_list")?
      .remove(index);

    Ok(Some(book))
  }

  fn remove_from_pending(
    recs: &mut LibraryRecommendations, book: &Book
  ) -> AppResult<Option<Book>> {
    let Some(index) = recs
      .pending
      .value()
      .conflict_if_none("pending_list")?
      .iter()
      .position(|b| b.id == book.id)
    else {
      return Ok(None);
    };

    let book = recs
      .pending
      .value_mut()
      .conflict_if_none("pending_list")?
      .remove(index);

    Ok(Some(book))
  }
}
