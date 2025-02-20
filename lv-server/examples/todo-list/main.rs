use lv_server::deps::actix_web;

pub mod prelude;
pub mod views;

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

fn routes(cfg: &mut actix_web::web::ServiceConfig) {
  use lv_server::View;

  views::shared::fragments_router(cfg);
  views::ViewHome::router(cfg);

  cfg.service(actix_files::Files::new("/static", "./examples/static"));
}
