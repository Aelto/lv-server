use crate::prelude::*;

#[lv_server::async_trait]
impl lv_server::PathExtractor for Library {
  type Params = String;

  const ID: &'static str = "PELibrary";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<String> {
    Self::param_from_str(req, "library_id")
  }

  async fn from_params(params: String) -> Option<Self> {
    Library::find_by_id(&params).unwrap()
  }
}

#[lv_server::async_trait]
impl lv_server::PathExtractor for Book {
  type Params = String;

  const ID: &'static str = "PEBook";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    Self::param_from_str(req, "book_id")
  }

  async fn from_params(params: String) -> Option<Self> {
    Book::find_by_id(&params).unwrap()
  }
}

#[lv_server::async_trait]
impl lv_server::PathExtractor for Author {
  type Params = String;

  const ID: &'static str = "PEAuthor";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    Self::param_from_str(req, "author_id")
  }

  async fn from_params(params: String) -> Option<Self> {
    Author::find_by_id(&params).unwrap()
  }
}
