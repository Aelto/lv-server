pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

pub mod db;
pub mod fragments;
pub mod models;
pub mod page;
pub mod views;

pub mod prelude;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  use actix_web::App;
  use actix_web::HttpServer;

  let port = 3000;

  db::init();

  println!("running server on http://localhost:{port}");

  HttpServer::new(move || App::new().configure(routes))
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

fn routes(cfg: &mut actix_web::web::ServiceConfig) {
  // use actix_web::web::get;

  cfg
    .configure(<views::ViewIndex as lv_server::WithRouter>::router)
    .service(actix_files::Files::new("/static", "./static"));
}
