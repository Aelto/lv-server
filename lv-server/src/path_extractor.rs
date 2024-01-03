use std::{future::Future, pin::Pin};

use actix_web::error::{ErrorConflict, ErrorNotFound};

#[async_trait::async_trait]
pub trait PathExtractor: Sized {
  type Params: Sized;

  fn identifier() -> &'static str;
  fn params(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Option<Self::Params>;

  async fn from_params(params: Self::Params) -> Option<Self>;
}

pub struct Need<PE>(pub PE);

impl<PE: 'static> actix_web::FromRequest for Need<PE>
where
  PE: PathExtractor
{
  type Error = actix_web::Error;

  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Self::Future {
    let Some(params) = PE::params(req, payload) else {
      let err = Err(ErrorConflict(""));

      return Box::pin(std::future::ready(err));
    };

    Box::pin(async move {
      match PE::from_params(params).await {
        Some(data) => Ok(Need(data)),
        None => Err(ErrorNotFound(PE::identifier()))
      }
    })
  }
}

// #[async_trait::async_trait]
// pub trait PathExtractor<Params>: Sized {
//   fn identifier() -> &'static str;
//   fn params(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Option<Params>;

//   async fn from_params(params: Params) -> Option<Self>;
// }

//
//
//

// pub struct Extract<PE, PARAMS> {
//   pub inner: PE,
//   params: PhantomData<PARAMS>
// }

// impl<PE, P> From<PE> for Extract<PE, P> {
//   fn from(value: PE) -> Self {
//     Self {
//       inner: value,
//       params: PhantomData::default()
//     }
//   }
// }

// impl<PE: 'static, P: 'static> actix_web::FromRequest for Extract<PE, P>
// where
//   PE: PathExtractor<P>
// {
//   type Error = actix_web::Error;

//   type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//   fn from_request(
//     req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
//   ) -> Self::Future {
//     let Some(params) = PE::params(req, payload) else {
//       let err = Err(ErrorConflict(""));

//       return Box::pin(std::future::ready(err));
//     };

//     Box::pin(async move {
//       match PE::from_params(params).await {
//         Some(data) => Ok(Extract::from(data)),
//         None => Err(ErrorNotFound(PE::identifier()))
//       }
//     })
//   }
// }

// impl LibraryPathExt {
//   fn params(
//     req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload
//   ) -> Result<String, actix_web::Error> {
//     let Some(id) = req.match_info().get("library_id") else {
//       return Err(ErrorConflict(
//         "Path: library id expected but not found".to_owned()
//       ));
//     };

//     Ok(id.to_owned())
//   }

//   async fn from_info(lib_id: String) -> Result<Self, actix_web::Error> {
//     let Some(library) = Library::find_by_id(&lib_id).unwrap() else {
//       return Err(ErrorNotFound(
//         "Path: no record found with library id".to_owned()
//       ));
//     };

//     Ok(Self(library))
//   }
// }
