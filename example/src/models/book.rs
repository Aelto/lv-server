use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
  pub title: String,
  pub content: String
}

impl Book {
  pub fn find_all() -> AppResult<Vec<Book>> {
    Ok(db::read("books")?.unwrap_or_default())
  }

  pub fn add(self) -> AppResult<()> {
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
