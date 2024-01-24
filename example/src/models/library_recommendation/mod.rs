use crate::prelude::*;

mod crud;
mod domain;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LibraryRecommendationStatus {
  Pending,
  Approved,
  Denied
}

impl Default for LibraryRecommendationStatus {
  fn default() -> Self {
    Self::Pending
  }
}

/// Combination of an edge record with a custom ID to allow for fast traversal
/// from either a book or a library and also O(1) querying for a book/library
/// combination
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BookRecommendation {
  #[allow(unused)]
  #[serde(skip_serializing)]
  id: Id,

  #[serde(alias = "out")]
  #[serde(skip_serializing)]
  pub library: ForeignKey<Library, Id>,

  #[serde(alias = "in")]
  #[serde(skip_serializing)]
  pub book: ForeignKey<Book, Id>,

  pub status: LibraryRecommendationStatus
}

surreal_simple_querybuilder::model!(LibraryRecommendation { id, status });
pub use schema::model;
crate::with_model!(BookRecommendation);

impl BookRecommendation {
  pub fn custom_id(book: &Book, library: &Library) -> Id {
    let id = format!("{}/{}", book.id(), library.id());

    Id::new_thing(model.to_string(), id)
  }

  /// utility function to get an iterator of books with the given status
  fn filter_books<'a>(
    recommendations: &'a Vec<Self>, status: &'a LibraryRecommendationStatus
  ) -> impl Iterator<Item = &'a Book> {
    recommendations
      .iter()
      .filter(|rec| rec.is(status))
      .filter_map(|rec| rec.book.value())
  }

  pub fn is(&self, status: &LibraryRecommendationStatus) -> bool {
    self.status.eq(&status)
  }

  /// Split a single vec of recommendations into three streams of books based on
  /// their status.
  pub fn split_by_status<'a>(
    recommendations: &'a Vec<Self>
  ) -> (
    impl Iterator<Item = &'a Book>,
    impl Iterator<Item = &'a Book>,
    impl Iterator<Item = &'a Book>
  ) {
    (
      Self::filter_books(&recommendations, &LibraryRecommendationStatus::Pending),
      Self::filter_books(&recommendations, &LibraryRecommendationStatus::Approved),
      Self::filter_books(&recommendations, &LibraryRecommendationStatus::Denied)
    )
  }
}
