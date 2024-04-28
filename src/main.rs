mod api;
mod controllers;
mod db;
mod utils;

use actix_web::{web, App, HttpResponse, HttpServer};
use controllers::router::register_router;
use db::mongodb::form_connection;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let _ = form_connection().await;

    // let server_port = env::var("PORT")
    //     .expect("Error: There is no PORT configuration in the .env file.");
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(|| HttpResponse::Ok()))
            .service(web::scope("/api/computation-service").configure(register_router))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
