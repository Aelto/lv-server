use crate::prelude::*;

static TABLE: &'static str = "libraries";

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
  #[serde(default)]
  pub id: String,
  pub title: String,

  #[serde(default)]
  pub documents: Vec<Book>
}

impl Library {
  pub fn find_all() -> AppResult<Vec<Self>> {
    Ok(db::read(TABLE)?.unwrap_or_default())
  }

  pub fn add(mut self) -> AppResult<()> {
    self.id = nanoid::nanoid!();

    let mut all = Self::find_all()?;
    all.push(self);
    db::write(TABLE.to_owned(), &all)?;

    Ok(())
  }

  pub fn find_by_title(title: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.title == title))
  }
}
