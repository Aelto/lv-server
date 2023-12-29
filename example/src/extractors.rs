use std::{future::Future, pin::Pin};

use actix_web::error::{ErrorConflict, ErrorNotFound};

use crate::prelude::*;

pub struct LibraryPathExt(pub Library);
impl actix_web::FromRequest for LibraryPathExt {
  type Error = actix_web::Error;

  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
    let id = req.match_info().get("library_id");

    let res = match id {
      None => Err(ErrorConflict(
        "Path: library id expected but not found".to_owned()
      )),
      Some(id) => {
        let Some(library) = Library::find_by_id(&id).unwrap() else {
          return Box::pin(std::future::ready(Err(ErrorNotFound(
            "Path: no record found with library id".to_owned()
          ))));
        };

        Ok(Self(library))
      }
    };

    Box::pin(std::future::ready(res))
  }
}
