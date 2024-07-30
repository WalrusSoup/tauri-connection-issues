// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

fn main() {
  println!("Hello, world!");
  // launch another thread with server
  let _server_thread = std::thread::spawn(|| {
    println!("Attempting to start actix server");
    let _ = server();
  });
  app_lib::run();
}

#[actix_web::main]
async fn server() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8888");
    HttpServer::new(|| {
        App::new()
            .route("/uwu", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
  // sleep for 280 seconds
  std::thread::sleep(std::time::Duration::from_secs(280));
  HttpResponse::Ok().body("Hey there!")
}