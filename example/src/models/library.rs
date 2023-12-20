use crate::prelude::*;

static TABLE: &'static str = "libraries";

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
  #[serde(default)]
  pub id: String,

  pub title: String
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

  pub fn find_by_id(id: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.id == id))
  }

  pub fn find_by_title(title: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.title == title))
  }

  pub fn books(&self) -> AppResult<Vec<Book>> {
    let books = Book::find_all().unwrap();

    Ok(
      books
        .into_iter()
        .filter(|b| b.fk_library == self.id)
        .collect()
    )
  }

  pub fn book(&self, id: &str) -> AppResult<Option<Book>> {
    let books = self.books().unwrap();

    Ok(books.into_iter().find(|b| b.id == id))
  }
}
