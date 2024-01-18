use crate::prelude::*;

static TABLE: &'static str = "books";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Book {
  #[serde(default)]
  pub id: String,

  pub title: String,
  pub content: String,
  pub created_at: chrono::DateTime<chrono::Utc>,

  pub fk_library: String
}

impl Book {
  pub async fn is_author(&self, author_id: &str) -> AppResult<bool> {
    let Some(library) = Library::find_by_id(&self.fk_library)? else {
      return Ok(false);
    };

    Ok(library.fk_author == author_id)
  }
}

impl Book {
  pub fn find_all() -> AppResult<Vec<Book>> {
    Ok(db::read(TABLE)?.unwrap_or_default())
  }

  pub async fn find_most_recent(page_size: usize) -> AppResult<Vec<Book>> {
    let mut all = Self::find_all()?;
    all.sort_by(|a, b| {
      a.created_at
        .partial_cmp(&b.created_at)
        .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(all.into_iter().take(page_size).collect())
  }

  pub fn find_by_id(id: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.id == id))
  }

  pub fn add(mut self, library: String) -> AppResult<()> {
    self.id = nanoid::nanoid!();
    self.fk_library = library;
    self.created_at = chrono::Utc::now();

    let mut all = Self::find_all()?;
    all.push(self);
    db::write(TABLE.to_owned(), &all)?;

    Ok(())
  }

  pub fn update(&self) -> AppResult<()> {
    let books = Self::find_all().unwrap();
    let mut books: Vec<Self> = books.into_iter().filter(|b| b.id != self.id).collect();

    books.push(self.clone());
    db::write(TABLE.to_owned(), &books)?;

    Ok(())
  }

  pub fn delete(self) -> AppResult<()> {
    let books = Self::find_all().unwrap();
    let books: Vec<Self> = books.into_iter().filter(|b| b.id != self.id).collect();

    db::write(TABLE.to_owned(), &books)?;

    Ok(())
  }
}

impl maud::Render for Book {
  fn render(&self) -> maud::Markup {
    let rendered_markdown = {
      use ammonia::clean;
      use pulldown_cmark::html::push_html;
      use pulldown_cmark::Options;
      use pulldown_cmark::Parser;

      let mut options = Options::empty();
      options.insert(Options::ENABLE_TABLES);
      options.insert(Options::ENABLE_STRIKETHROUGH);
      options.insert(Options::ENABLE_TASKLISTS);

      let md_parse = Parser::new_ext(&self.content, options);
      let mut unsafe_html = String::new();
      push_html(&mut unsafe_html, md_parse);

      let safe_html = clean(&*unsafe_html);

      safe_html
    };

    html!(
      div.title {(self.title)}
      div.content {(maud::PreEscaped(rendered_markdown))}
    )
  }
}
