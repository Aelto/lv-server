use std::{future::Future, pin::Pin};

use actix_web::error::{ErrorConflict, ErrorNotFound};

#[async_trait::async_trait]
pub trait PathExtractor: Sized {
  type Params: Sized;

  const ID: &'static str;
  fn params(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Option<Self::Params>;

  async fn from_params(params: Self::Params) -> Option<Self>;

  /// Extracts a value into an owned String from the given param name
  fn param_from_str(req: &actix_web::HttpRequest, param: &str) -> Option<String> {
    req.match_info().get(param).map(str::to_owned)
  }
}

////////////////////////////////////////////////////////////////////////////////

#[async_trait::async_trait]
impl<PE1, PE2> PathExtractor for (PE1, PE2)
where
  PE1: PathExtractor + Send,
  PE2: PathExtractor + Send,
  PE1::Params: Send,
  PE2::Params: Send
{
  type Params = (PE1::Params, PE2::Params);

  const ID: &'static str = "PathExtractor(PE1,PE2)";

  fn params(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Option<Self::Params> {
    let Some(p1) = PE1::params(req, payload) else {
      return None;
    };

    let Some(p2) = PE2::params(req, payload) else {
      return None;
    };

    Some((p1, p2))
  }

  async fn from_params(params: Self::Params) -> Option<Self> {
    let Some(p1) = PE1::from_params(params.0).await else {
      return None;
    };

    let Some(p2) = PE2::from_params(params.1).await else {
      return None;
    };

    Some((p1, p2))
  }
}

#[async_trait::async_trait]
impl<PE1, PE2, PE3> PathExtractor for (PE1, PE2, PE3)
where
  PE1: PathExtractor + Send,
  PE2: PathExtractor + Send,
  PE3: PathExtractor + Send,
  PE1::Params: Send,
  PE2::Params: Send,
  PE3::Params: Send
{
  type Params = (PE1::Params, PE2::Params, PE3::Params);

  const ID: &'static str = "PathExtractor(PE1,PE2)";

  fn params(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Option<Self::Params> {
    let Some(p1) = PE1::params(req, payload) else {
      return None;
    };

    let Some(p2) = PE2::params(req, payload) else {
      return None;
    };

    let Some(p3) = PE3::params(req, payload) else {
      return None;
    };

    Some((p1, p2, p3))
  }

  async fn from_params(params: Self::Params) -> Option<Self> {
    let Some(p1) = PE1::from_params(params.0).await else {
      return None;
    };

    let Some(p2) = PE2::from_params(params.1).await else {
      return None;
    };

    let Some(p3) = PE3::from_params(params.2).await else {
      return None;
    };

    Some((p1, p2, p3))
  }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Need<PE>(pub PE);

impl<PE> Need<PE>
where
  PE: PathExtractor
{
  pub fn into_inner(self) -> PE {
    self.0
  }
}

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
      let err = Err(ErrorConflict(PE::ID));

      return Box::pin(std::future::ready(err));
    };

    Box::pin(async move {
      match PE::from_params(params).await {
        Some(data) => Ok(Need(data)),
        None => Err(ErrorNotFound(PE::ID))
      }
    })
  }
}
