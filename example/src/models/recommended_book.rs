use nanoid::nanoid;

use crate::prelude::*;

static TABLE: &'static str = "recommeded-books";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecommendedBook {
  #[serde(default)]
  pub id: String,

  pub fk_library: String,
  pub fk_author: String,
  pub fk_book: String
}

impl RecommendedBook {
  pub async fn add(mut self, library: String, author: String, book: String) -> AppResult<()> {
    self.id = nanoid!();
    self.fk_author = author;
    self.fk_book = book;
    self.fk_library = library;

    let mut all = Self::find_all()?;
    all.push(self);
    db::write(TABLE.to_owned(), &all)?;

    Ok(())
  }

  pub fn find_all() -> AppResult<Vec<RecommendedBook>> {
    Ok(db::read(TABLE)?.unwrap_or_default())
  }

  pub fn find_by_id(id: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.id == id))
  }

  pub async fn find_all_by_library(library: &str) -> AppResult<Vec<Self>> {
    let all = Self::find_all()?;

    Ok(
      all
        .into_iter()
        .filter(|rb| rb.fk_library == library)
        .collect()
    )
  }

  pub async fn book(&self) -> AppResult<Option<Book>> {
    Book::find_by_id(&self.fk_book)
  }

  pub fn update(&self) -> AppResult<()> {
    let all = Self::find_all().unwrap();
    let mut filtered: Vec<Self> = all.into_iter().filter(|b| b.id != self.id).collect();

    filtered.push(self.clone());
    db::write(TABLE.to_owned(), &filtered)?;

    Ok(())
  }

  pub async fn delete(self) -> AppResult<()> {
    let all = Self::find_all().unwrap();
    let filtered: Vec<Self> = all.into_iter().filter(|b| b.id != self.id).collect();

    db::write(TABLE.to_owned(), &filtered)?;

    Ok(())
  }
}

impl maud::Render for RecommendedBook {
  fn render(&self) -> maud::Markup {
    html!(
      div {
        div.title {"RecommendedBook(author = "(self.fk_author)", book = "(self.fk_book)")"}
      }
    )
  }
}
