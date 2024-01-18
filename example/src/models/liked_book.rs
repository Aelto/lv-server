use crate::prelude::*;

static TABLE: &'static str = "liked-books";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LikedBook {
  #[serde(default)]
  pub id: String,

  pub fk_author: String,
  pub fk_book: String
}

impl LikedBook {
  pub async fn does_like_book(author: &str, book: &str) -> AppResult<bool> {
    Ok(Self::find_by_author_and_book(author, book).await?.is_some())
  }
}

impl LikedBook {
  pub async fn book(&self) -> AppResult<Option<Book>> {
    Book::find_by_id(&self.fk_book)
  }

  pub async fn as_books(liked_books: &Vec<Self>) -> AppResult<Vec<Book>> {
    let mut out = Vec::with_capacity(liked_books.len());

    for like in liked_books {
      if let Some(book) = Book::find_by_id(&like.fk_book)? {
        out.push(book);
      }
    }

    Ok(out)
  }
}

impl LikedBook {
  pub fn find_all() -> AppResult<Vec<LikedBook>> {
    Ok(db::read(TABLE)?.unwrap_or_default())
  }

  pub fn find_by_id(id: &str) -> AppResult<Option<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().find(|l| l.id == id))
  }

  pub async fn find_by_author(author: &str) -> AppResult<Vec<Self>> {
    let all = Self::find_all()?;

    Ok(all.into_iter().filter(|l| l.fk_author == author).collect())
  }

  pub async fn find_by_author_and_book(author: &str, book: &str) -> AppResult<Option<Self>> {
    let target_id = format!("{author}/{book}");

    Self::find_by_id(&target_id)
  }

  pub fn add(mut self, author: String, book: String) -> AppResult<()> {
    self.id = format!("{author}/{book}");
    self.fk_author = author;
    self.fk_book = book;

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

  pub async fn delete(self) -> AppResult<()> {
    let books = Self::find_all().unwrap();
    let books: Vec<Self> = books.into_iter().filter(|b| b.id != self.id).collect();

    db::write(TABLE.to_owned(), &books)?;

    Ok(())
  }
}

impl maud::Render for LikedBook {
  fn render(&self) -> maud::Markup {
    html!(
      div {
        div.title {"LikedBook(author = "(self.fk_author)", book = "(self.fk_book)")"}
      }
    )
  }
}
