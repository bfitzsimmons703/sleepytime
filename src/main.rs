mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    HttpServer::new(|| {
        App::new()
            .service(handlers::sleep)
            .default_service(web::route().to(handlers::not_found))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
