use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
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

  pub fn add(mut self, library: String) -> AppResult<()> {
    self.id = nanoid::nanoid!();
    self.fk_library = library;

    let mut all = Self::find_all()?;
    all.push(self);
    db::write("books".to_owned(), &all)?;

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
