use crate::prelude::*;

pub struct LibraryPathExt(pub Library);

#[lv_server::async_trait]
impl lv_server::PathExtractor for LibraryPathExt {
  type Params = String;

  fn identifier() -> &'static str {
    "LibraryPathExt"
  }

  fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<String> {
    req.match_info().get("library_id").map(str::to_owned)
  }

  async fn from_params(params: String) -> Option<Self> {
    Library::find_by_id(&params).unwrap().map(|l| Self(l))
  }
}

pub struct LibraryBookPathExt(pub Library, pub Book);

#[lv_server::async_trait]
impl lv_server::PathExtractor for LibraryBookPathExt {
  type Params = (String, String);

  fn identifier() -> &'static str {
    "LibraryBookPathExt"
  }

  fn params(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Option<Self::Params> {
    LibraryPathExt::params(req, payload).and_then(|lib| {
      req
        .match_info()
        .get("book_id")
        .map(|book| (lib.to_owned(), book.to_owned()))
    })
  }

  async fn from_params((lib, book): Self::Params) -> Option<Self> {
    LibraryPathExt::from_params(lib)
      .await
      .map(|lib| lib.0)
      .and_then(|lib| lib.book(&book).unwrap().map(|book| Self(lib, book)))
  }
}
