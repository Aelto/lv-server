use crate::prelude::*;

static TABLE: &'static str = "authors";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Author {
  #[serde(default)]
  pub id: String,

  pub handle: String
}

impl Author {
  pub fn find_all() -> AppResult<Vec<Self>> {
    Ok(db::read(TABLE)?.unwrap_or_default())
  }

  pub fn add(mut self) -> AppResult<Self> {
    self.id = nanoid::nanoid!();

    let mut all = Self::find_all()?;
    all.push(self.clone());
    db::write(TABLE.to_owned(), &all)?;

    Ok(self)
  }

  pub fn find_by_id(id: &str) -> AppResult<Option<Self>> {
    Ok(Self::find_all()?.into_iter().find(|a| a.id == id))
  }

  pub fn libraries(&self) -> AppResult<Vec<Library>> {
    Library::find_by_author(&self.id)
  }
}
