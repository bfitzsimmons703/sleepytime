mod handlers;

use actix_web::{web, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .service(handlers::sleep)
            .default_service(web::route().to(handlers::not_found))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
