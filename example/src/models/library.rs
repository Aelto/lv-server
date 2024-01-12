use crate::prelude::*;

static TABLE: &'static str = "libraries";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Library {
  #[serde(default)]
  pub id: String,

  pub title: String,

  pub fk_author: String
}

impl Library {
  pub fn find_all() -> AppResult<Vec<Self>> {
    Ok(db::read(TABLE)?.unwrap_or_default())
  }

  pub fn add(mut self, author: String) -> AppResult<Self> {
    self.id = nanoid::nanoid!();
    self.fk_author = author;

    let mut all = Self::find_all()?;
    all.push(self.clone());
    db::write(TABLE.to_owned(), &all)?;

    Ok(self)
  }

  pub fn find_by_id(id: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.id == id))
  }

  pub fn find_by_title(title: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.title == title))
  }

  pub fn find_by_author(author: &str) -> AppResult<Vec<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().filter(|l| l.fk_author == author).collect())
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
