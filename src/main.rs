mod handlers;

use std::env;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "80".to_owned())
        .parse()
        .expect("PORT must be a number");

    info!("Listening on port {}", port);
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(handlers::sleep)
            .default_service(web::route().to(handlers::not_found))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
