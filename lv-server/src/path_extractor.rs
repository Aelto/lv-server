use std::{future::Future, pin::Pin};

use actix_web::error::{ErrorConflict, ErrorNotFound};

/// A type can implement [PathExtractor] to be retrieved (from a database
/// or any storage or from memory) using a segment of an endpoint's route.
///
/// Once wrapped in a [Need], this offers a convenient wrapper around
/// Actix' [actix_web::FromRequest] trait.
///
/// ```
/// #[async_trait::async_trait]
/// impl lv_server::PathExtractor for Todo {
///   type Params = String;
///
///   const ID: &'static str = "PE_Todo";
///   fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
///     req
///       .match_info()
///       .get("todo_id")
///       .map(|id| id.to_string())
///   }
///
///   async fn from_params(params: String) -> Option<Self> {
///     DB.getTodoById(&params)
///   }
/// }
///
/// ```
///
/// For information on how to use them in endpoints, refer to [Need]
#[async_trait::async_trait]
pub trait PathExtractor: Sized {
  /// The types this extractor needs to obtain from the request.
  ///
  /// Can be anything as long as it's [Sized], even a tuple of multiple
  /// Sized types.
  ///
  /// ```rs
  /// type Params = String;
  ///
  /// // if more than a string is needed:
  /// type Params = (String, String);
  /// ```
  type Params: Sized;

  /// A unique identifier for this extractor, it's displayed during errors.
  /// ```rs
  /// const ID: &'static str = "MyExtractor";
  /// ```
  const ID: &'static str;

  /// This is where the params are constructed from the request (usually
  /// its path).
  ///
  /// ```rs
  /// fn params(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Option<Self::Params> {
  ///     req
  ///       .match_info()
  ///       .get("todo_id")
  ///       .map(|id| id.to_string())
  ///   }
  /// ```
  ///
  /// It can be useful to call another type's extractor to get
  /// progressively deeper elements. For example in the case of an
  /// `Account` with multiple `Projects`:
  /// ```rs
  /// type Params = (Slug, Slug);
  ///
  /// fn params(
  ///   req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload,
  /// ) -> Option<Self::Params> {
  ///   // call the Account's extractor:
  ///   Account::params(req, payload)
  ///   // then append this project's Slug if it can be found:
  ///   .zip(req.match_info().get("project_slug").map(Slug::from))
  /// }
  /// ```
  fn params(
    req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload
  ) -> Option<Self::Params>;

  /// Where the element is retrieved from the storage/DB or constructed
  /// using the parameters that were obtained from the request.
  ///
  /// If the parameters could not be found then this function is never
  /// called and an HTTP CONFLICT is returned with the Extractor's ID as
  /// the unique information about the reason.
  ///
  /// ```rs
  /// async fn from_params(params: String) -> Option<Self> {
  ///   DB.getTodoById(&params)
  /// }
  /// ```
  async fn from_params(params: Self::Params) -> Option<Self>;

  /// Utility function: extracts a value into an owned String from the given param name
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

/// A [PathExtractor] wrapped in a [Need] can be used in Actix endpoints to
/// specify elements that must be constructed from the current request (
/// usually from its path).
///
/// ```rs
/// struct Todo {
///   id: String,
///   text: String
/// }
///
/// #[async_trait::async_trait]
/// impl lv_server::PathExtractor for Todo {
///   // ...
/// }
///
/// impl api::delete_todo::Router {
///   pub async fn endpoint(Need(todo): Need<Todo>) -> HttpResponse {
///     todo.delete();
///
///     // ...
///   }
/// }
/// ```
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
