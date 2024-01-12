use crate::prelude::*;

pub struct PELibrary(pub Library);

#[lv_server::async_trait]
impl lv_server::PathExtractor for PELibrary {
  type Params = String;

  const ID: &'static str = "PELibrary";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<String> {
    Self::param_from_str(req, "library_id")
  }

  async fn from_params(params: String) -> Option<Self> {
    Library::find_by_id(&params).unwrap().map(|l| Self(l))
  }
}

pub struct PEBook(pub Book);

#[lv_server::async_trait]
impl lv_server::PathExtractor for PEBook {
  type Params = String;

  const ID: &'static str = "PEBook";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    Self::param_from_str(req, "book_id")
  }

  async fn from_params(params: String) -> Option<Self> {
    Book::find_by_id(&params).unwrap().map(|l| Self(l))
  }
}

pub struct PEAuthor(pub Author);

#[lv_server::async_trait]
impl lv_server::PathExtractor for PEAuthor {
  type Params = String;

  const ID: &'static str = "PEAuthor";
  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
    Self::param_from_str(req, "author_id")
  }

  async fn from_params(params: String) -> Option<Self> {
    Author::find_by_id(&params).unwrap().map(|l| Self(l))
  }
}
