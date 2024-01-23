use crate::prelude::*;

use self::library_recommendations::LibraryRecommendationsParams;

#[lv_server::async_trait]
impl lv_server::PathExtractor for Library {
  type Params = Id;

  const ID: &'static str = "PELibrary";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    req
      .match_info()
      .get("library_id")
      .map(|uuid| Id::new_thing(models::library::model.to_string(), uuid))
  }

  async fn from_params(params: Id) -> Option<Self> {
    Library::find_by_id(&params, LibraryParams::None)
      .await
      .unwrap_or_default()
  }
}

pub struct LibraryWithBooks(pub Library);
#[lv_server::async_trait]
impl lv_server::PathExtractor for LibraryWithBooks {
  type Params = Id;

  const ID: &'static str = "PELibraryWithBooks";
  fn params(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Option<Self::Params> {
    <Library as lv_server::PathExtractor>::params(req, payload)
  }

  async fn from_params(params: Id) -> Option<Self> {
    Library::find_by_id(&params, LibraryParams::FetchBooks)
      .await
      .unwrap_or_default()
      .map(|a| LibraryWithBooks(a))
  }
}

#[lv_server::async_trait]
impl lv_server::PathExtractor for Book {
  type Params = Id;

  const ID: &'static str = "PEBook";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    req
      .match_info()
      .get("book_id")
      .map(|uuid| Id::new_thing(models::book::model.to_string(), uuid))
  }

  async fn from_params(params: Id) -> Option<Self> {
    Book::find_by_id(&params, BookParams::None)
      .await
      .unwrap_or_default()
  }
}

#[lv_server::async_trait]
impl lv_server::PathExtractor for Author {
  type Params = Id;

  const ID: &'static str = "PEAuthor";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    req
      .match_info()
      .get("author_id")
      .map(|uuid| Id::new_thing(models::author::model.to_string(), uuid))
  }

  async fn from_params(params: Id) -> Option<Self> {
    Author::find_by_id(&params, AuthorParams::None)
      .await
      .unwrap_or_default()
  }
}

pub struct AuthorWithLibraries(pub Author);
#[lv_server::async_trait]
impl lv_server::PathExtractor for AuthorWithLibraries {
  type Params = Id;

  const ID: &'static str = "PEAuthorWithLibraries";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    req
      .match_info()
      .get("author_id")
      .map(|uuid| Id::new_thing(models::author::model.to_string(), uuid))
  }

  async fn from_params(params: Id) -> Option<Self> {
    Author::find_by_id(&params, AuthorParams::FetchLibraries)
      .await
      .unwrap_or_default()
      .map(|a| AuthorWithLibraries(a))
  }
}

#[lv_server::async_trait]
impl lv_server::PathExtractor for LibraryRecommendations {
  type Params = Id;

  const ID: &'static str = "PELibraryRecommendations";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    req
      .match_info()
      .get("library_recommendations_id")
      .map(|uuid| Id::new_thing(models::library_recommendations::model.to_string(), uuid))
  }

  async fn from_params(params: Id) -> Option<Self> {
    Self::find_by_id(&params, LibraryRecommendationsParams::None)
      .await
      .unwrap_or_default()
  }
}
