mod database;
mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/human", web::get().to(get_humans))
            .route("/human/{idd}", web::get().to(get_human))
            .route("/human", web::post().to(create_human))
            .route("/human/{idd}", web::put().to(update_human))
            .route("/human/{idd}", web::delete().to(delete_human))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
