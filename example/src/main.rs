pub mod db;
pub mod dev;
pub mod extractors;
pub mod exts;
pub mod fragments;
pub mod models;
pub mod page;
pub mod result;
pub mod views;

pub mod prelude;

#[tokio::main]
async fn main() -> result::AppResult<()> {
  use actix_web::App;
  use actix_web::HttpServer;

  let port = 3000;

  db::connect("address", "username", "password", "namespace", "database").await?;

  println!("running server on http://localhost:{port}");

  HttpServer::new(move || App::new().configure(routes))
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await?;

  Ok(())
}

fn routes(cfg: &mut actix_web::web::ServiceConfig) {
  use lv_server::View;

  views::ViewHome::router(cfg);
  views::ViewProfile::router(cfg);
  views::ViewProfileLibrary::router(cfg);

  cfg.service(actix_files::Files::new("/static", "./static"));
}
