use surreal_simple_querybuilder::queries::QueryBuilderInjecter;
use surreal_simple_querybuilder::types::Fetch;

/// A custom type that is used to abstract away the SQL params so we don't have
/// to use the model in the API layer
#[derive(Clone, Copy)]
pub enum LibraryRecommendationsParams {
  None,
  FetchLibrary,
  FetchApproved,
  FetchToApprove,
  FetchDenied,
  FetchFull
}

impl<'a> QueryBuilderInjecter<'a> for LibraryRecommendationsParams {
  fn inject(
    &self, querybuilder: surreal_simple_querybuilder::querybuilder::QueryBuilder<'a>
  ) -> surreal_simple_querybuilder::querybuilder::QueryBuilder<'a> {
    use crate::models::library_recommendations::model;

    match self {
      Self::None => ().inject(querybuilder),
      Self::FetchLibrary => Fetch([&*model.library]).inject(querybuilder),
      Self::FetchApproved => Fetch([&*model.approved]).inject(querybuilder),
      Self::FetchToApprove => Fetch([&*model.pending]).inject(querybuilder),
      Self::FetchDenied => Fetch([&*model.denied]).inject(querybuilder),
      Self::FetchFull => Fetch([
        &*model.library,
        &*model.approved,
        &*model.pending,
        &*model.denied
      ])
      .inject(querybuilder)
    }
  }
}
