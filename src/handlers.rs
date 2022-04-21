use std::time::Duration;

use actix_web::{get, web, HttpResponse};
use tokio::time;

#[get("/sleep/{ms}")]
pub async fn sleep(path: web::Path<u64>) -> HttpResponse {
    let millis = path.into_inner();
    
    if millis > 15000 {
        return HttpResponse::BadRequest().body("ms must be 0 - 15000");
    }

    time::sleep(Duration::from_millis(millis)).await; //non-blocking
    HttpResponse::Ok().finish()
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Not Found</h1>")
}
