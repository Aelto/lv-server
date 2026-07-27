use actix_web::FromRequest;
use lv_server::deps::actix_web;

pub mod prelude;
pub mod views;
pub mod components;

mod app_data;
mod page;



#[tokio::main]
async fn main() {
  use actix_web::App;
  use actix_web::HttpServer;

  let port = 3000;

  println!("running server on http://localhost:{port}");

  let app_data = actix_web::web::Data::new(app_data::AppData::new());

  HttpServer::new(move || {
    App::new()
      .app_data(actix_web::web::Data::clone(&app_data))
      .configure(routes)
  })
  .bind(format!("127.0.0.1:{}", port))
  .expect("HTTP server failure: local port unavailable")
  .run()
  .await
  .expect("failed to boot actix HTTP server");
}

use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::sync::LazyLock;
use std::collections::HashMap;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Path;

pub type Handler = Box<
  dyn Fn(actix_web::HttpRequest, actix_web::web::Payload)
          -> Pin<Box<dyn Future<Output = HttpResponse>>>
          + Send
          + Sync,
>;

pub static RESOURCES: LazyLock<
  Mutex<
    HashMap<
      String,
      Handler
    >
  >
> = LazyLock::new(|| Mutex::new(HashMap::new()));

// setting up a view in the main Actix app:
fn routes(cfg: &mut actix_web::web::ServiceConfig) {
  use lv_server::View;

  views::shared::fragments_router(cfg);

  // this sets up the View itself, but also any fragment it may have:
  views::ViewHome::router(cfg);

  components::paginated_todos::PaginatedFakeItem::router(cfg);
  async fn handler(request: HttpRequest, body: actix_web::web::Payload, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let lock = RESOURCES.lock().unwrap();
    if let Some(handler) = lock.get(&id) {
      let response = handler(request, body);

      return response.await
    }

    panic!()
  }
  cfg.route("/lv-server/anonymous/{id}", actix_web::web::post().to(handler));

  cfg.service(actix_files::Files::new("/static", "./examples/static"));
}
