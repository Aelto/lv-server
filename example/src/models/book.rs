use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
  #[serde(default)]
  pub id: String,

  pub title: String,

  #[serde(default)]
  pub content: String,

  #[serde(default)]
  pub fk_library: String
}

impl Book {
  pub fn find_all() -> AppResult<Vec<Book>> {
    Ok(db::read("books")?.unwrap_or_default())
  }

  pub fn find_by_id(id: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.id == id))
  }

  pub fn add(mut self, library: String) -> AppResult<()> {
    self.id = nanoid::nanoid!();
    self.fk_library = library;

    let mut all = Self::find_all()?;
    all.push(self);
    db::write("books".to_owned(), &all)?;

    Ok(())
  }

  pub fn update(&self) -> AppResult<()> {
    let books = Self::find_all().unwrap();
    let mut books: Vec<Self> = books.into_iter().filter(|b| b.id != self.id).collect();

    books.push(self.clone());
    db::write("books".to_owned(), &books)?;

    Ok(())
  }
}

impl maud::Render for Book {
  fn render(&self) -> maud::Markup {
    html!(
      div {
        div.title {"Book(name = "(self.title)")"}
        div.content {(self.content)}
      }
    )
  }
}
