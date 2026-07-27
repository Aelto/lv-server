use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::sync::LazyLock;
use std::collections::HashMap;
use actix_web::HttpResponse;

pub type Handler =
  fn(actix_web::HttpRequest, actix_web::web::Payload)
          -> Pin<Box<dyn Future<Output = HttpResponse>>>;

pub(crate) static HANDLER_REGISTRY: LazyLock<
  Mutex<
    HashMap<
      String,
      Handler
    >
  >
> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn add_handler_if_missing(key: String, handler: Handler) {
  if let Ok(mut lock) = HANDLER_REGISTRY.lock() {
    if !lock.contains_key(&key) {
      lock.insert(key, handler);
    }
  }
}
