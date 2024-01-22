pub use error::AppError;

/// A result that can hold an internal API error, this type shouldn't be used
/// on user facing endpoints directly but rather internally for passing around
/// data with an error that is handled by the server.
///
/// Refer to [ApiResponse] for user facing endpoints.
pub type AppResult<T> = Result<T, AppError>;

/// A response coming from a backend server, this type should be used on all
/// user facing endpoints.
pub type AppResponse = AppResult<actix_web::HttpResponse>;

/// To be used in failible SSR templating functions as a return type.
///
/// Can be easily transformed into an ApiResponse by doing:
/// ```rs
///
/// responses::html(template_response?.into_string())
/// ```
pub type TemplateResponse = AppResult<maud::Markup>;

mod error {
  use std::fmt::Debug;
  use std::fmt::Display;

  use actix_web::http::StatusCode;

  use super::responses::*;

  /// The error type used **everywhere** in the backend servers
  #[derive(Debug)]
  pub enum AppError {
    Conflict(&'static str),
    Unauthorized(&'static str),
    NotFound(&'static str),
    InternalServerError(&'static str),

    Validation(String),

    /// A normal message, for when you want to return early in case of an error but
    /// it is still a valid path.
    Message(serde_json::Value),

    /// An empty error for when you want to early return in case of an error but
    /// it is still a valid path. This is mostly used in SSR templating code
    /// where some parts of the page (component functions) leave early in case
    /// of a `None` or `Err` value, and use the Render implementation of this
    /// Empty error to display nothing.
    Empty
  }

  impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        AppError::Conflict(s) => write!(f, "{s}"),
        AppError::Unauthorized(s) => write!(f, "{s}"),
        AppError::NotFound(s) => write!(f, "{s}"),
        AppError::InternalServerError(s) => write!(f, "{s}"),
        AppError::Validation(s) => write!(f, "{s}"),
        AppError::Message(s) => write!(f, "{s}"),
        AppError::Empty => Ok(())
      }
    }
  }

  impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
      match self {
        AppError::Conflict(_) => StatusCode::CONFLICT,
        AppError::NotFound(_) => StatusCode::NOT_FOUND,
        AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::Validation(_) => StatusCode::CONFLICT,
        AppError::Message(_) => StatusCode::OK,
        AppError::Empty => StatusCode::OK
      }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
      match self {
        AppError::Conflict(s) => _conflict(s),
        AppError::NotFound(s) => _not_found(s),
        AppError::Unauthorized(s) => _unauthorized(s),
        AppError::InternalServerError(s) => _internal_error(s),
        AppError::Validation(s) => _conflict(&s),
        AppError::Message(s) => _ok(s),
        AppError::Empty => _empty()
      }
    }
  }

  impl std::error::Error for AppError {}

  ////////////////////////////////////////////////////////////////////////////////

  impl maud::Render for AppError {
    fn render(&self) -> maud::Markup {
      match self {
        AppError::Conflict(message) => maud::html!("Conflict: "(message)),
        AppError::Unauthorized(message) => maud::html!("Unauthorized: "(message)),
        AppError::NotFound(message) => maud::html!("NotFound: "(message)),
        AppError::InternalServerError(message) => maud::html!("InternalServerError: "(message)),
        AppError::Validation(message) => maud::html!("Validation: "(message)),
        AppError::Message(message) => maud::html!("Message: "(message.to_string())),
        AppError::Empty => maud::html!()
      }
    }

    fn render_to(&self, buffer: &mut String) {
      buffer.push_str(&self.render().into_string());
    }
  }

  ////////////////////////////////////////////////////////////////////////////////
  // From implementations

  impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
      println!("error: {value}");

      Self::InternalServerError("database_error")
    }
  }

  impl From<surrealdb::error::Db> for AppError {
    fn from(value: surrealdb::error::Db) -> Self {
      surrealdb::Error::Db(value).into()
    }
  }

  impl From<surrealdb::error::Api> for AppError {
    fn from(value: surrealdb::error::Api) -> Self {
      surrealdb::Error::Api(value).into()
    }
  }

  impl From<actix_web::error::BlockingError> for AppError {
    fn from(value: actix_web::error::BlockingError) -> Self {
      println!("error: {value}");

      Self::InternalServerError("blocking_error")
    }
  }

  impl From<std::time::SystemTimeError> for AppError {
    fn from(value: std::time::SystemTimeError) -> Self {
      println!("error: {value}");

      Self::InternalServerError("systemtime_error")
    }
  }

  impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
      println!("error: {value}");

      Self::Conflict("serde_error")
    }
  }

  impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
      println!("error: {value}");

      Self::InternalServerError("io_error")
    }
  }

  impl From<&'static str> for AppError {
    fn from(value: &'static str) -> Self {
      Self::InternalServerError(value)
    }
  }

  impl From<surreal_simple_querybuilder::foreign_key::IntoKeyError> for AppError {
    fn from(value: surreal_simple_querybuilder::foreign_key::IntoKeyError) -> Self {
      use surreal_simple_querybuilder::foreign_key::IntoKeyError;

      match value {
        IntoKeyError::Custom(m) => Self::InternalServerError(m),
        IntoKeyError::MissingId => Self::InternalServerError("IntoKeyError: missing id"),
        IntoKeyError::TransformError => Self::InternalServerError("IntoKeyError: transform error")
      }
    }
  }

  impl<MessageType> From<tokio::sync::mpsc::error::SendError<MessageType>> for AppError
  where
    MessageType: Debug
  {
    fn from(value: tokio::sync::mpsc::error::SendError<MessageType>) -> Self {
      println!("tokio Send Error: {value:?}");

      Self::InternalServerError("mpsc_send_error")
    }
  }
}

#[allow(unused)]
mod responses {
  use super::AppResponse;
  use actix_web::HttpResponse;
  use serde::Serialize;
  use serde_json::json;

  pub fn ok<T>(body: T) -> AppResponse
  where
    T: Serialize
  {
    Ok(HttpResponse::Ok().json(body))
  }

  pub(crate) fn _ok<T>(body: T) -> HttpResponse
  where
    T: Serialize
  {
    HttpResponse::Ok().json(body)
  }

  #[allow(unused)]
  pub fn html(body: String) -> AppResponse {
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
  }

  #[allow(unused)]
  pub fn found(redirect_url: &str) -> AppResponse {
    Ok(_found(redirect_url))
  }

  pub(crate) fn _found(redirect_url: &str) -> HttpResponse {
    use actix_web::http::header::LOCATION;

    HttpResponse::Found()
      .append_header((LOCATION, redirect_url))
      .finish()
  }

  pub fn conflict(error_message: &str) -> AppResponse {
    Ok(_conflict(error_message))
  }

  pub(crate) fn _conflict(error_message: &str) -> HttpResponse {
    HttpResponse::Conflict().json(json!({ "error": error_message }))
  }

  pub fn unauthorized(error_message: &str) -> AppResponse {
    Ok(_unauthorized(error_message))
  }

  pub(crate) fn _unauthorized(error_message: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(json!({ "error": error_message }))
  }

  pub fn not_found(error_message: &str) -> AppResponse {
    Ok(HttpResponse::NotFound().json(json!({ "error": error_message })))
  }

  pub(crate) fn _not_found(error_message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(json!({ "error": error_message }))
  }

  pub fn internal_error(error_message: &str) -> AppResponse {
    Ok(_internal_error(error_message))
  }

  pub(crate) fn _internal_error(error_message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({ "error": error_message }))
  }

  pub fn created<T>(body: T) -> AppResponse
  where
    T: Serialize
  {
    Ok(HttpResponse::Created().json(body))
  }

  pub fn empty() -> AppResponse {
    Ok(_empty())
  }

  pub fn _empty() -> HttpResponse {
    HttpResponse::Ok().finish()
  }
}
