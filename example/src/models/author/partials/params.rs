use surreal_simple_querybuilder::queries::QueryBuilderInjecter;
use surreal_simple_querybuilder::types::Fetch;

/// A custom type that is used to abstract away the SQL params so we don't have
/// to use the model in the API layer
#[derive(Clone, Copy)]
pub enum AuthorParams {
  None,
  FetchLibraries
}

impl<'a> QueryBuilderInjecter<'a> for AuthorParams {
  fn inject(
    &self, querybuilder: surreal_simple_querybuilder::querybuilder::QueryBuilder<'a>
  ) -> surreal_simple_querybuilder::querybuilder::QueryBuilder<'a> {
    use crate::models::author::model;

    match self {
      Self::None => ().inject(querybuilder),
      Self::FetchLibraries => Fetch([&*model.libraries]).inject(querybuilder)
    }
  }
}
