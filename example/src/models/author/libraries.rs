use crate::prelude::*;

use super::model;

impl Author {
  pub async fn add_library_record(author_id: &Id, library_id: &Id) -> AppResult<()> {
    Self::m_add_one(author_id, &model.libraries, library_id).await?;

    Ok(())
  }

  pub async fn remove_library_record(author_id: &Id, library_id: &Id) -> AppResult<Self> {
    let author = Self::find_by_id(&author_id, AuthorParams::None)
      .await?
      .not_found_if_none("library_not_found")?;

    let filtered: Vec<Id> = author
      .libraries
      .into_inner()
      .into_key()
      .internal_error_if_none("author_missing_libraries")?
      .into_iter()
      .filter(|id| id != library_id)
      .collect();

    let project = Self::m_replace_one(author_id, &model.libraries, filtered).await?;

    Ok(project)
  }
}
