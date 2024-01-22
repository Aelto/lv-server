use surreal_simple_querybuilder::queries::QueryBuilderInjecter;
use surreal_simple_querybuilder::types::Fetch;

/// A custom type that is used to abstract away the SQL params so we don't have
/// to use the model in the API layer
#[derive(Clone, Copy)]
pub enum BookParams {
  None,
  FetchLibrary
}

impl<'a> QueryBuilderInjecter<'a> for BookParams {
  fn inject(
    &self, querybuilder: surreal_simple_querybuilder::querybuilder::QueryBuilder<'a>
  ) -> surreal_simple_querybuilder::querybuilder::QueryBuilder<'a> {
    use crate::models::book::model;

    match self {
      Self::None => ().inject(querybuilder),
      Self::FetchLibrary => Fetch([&*model.library]).inject(querybuilder)
    }
  }
}
