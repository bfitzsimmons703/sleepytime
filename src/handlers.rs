use actix_web::{get, HttpRequest, HttpResponse};
use std::time::Duration;
use tokio::time;

const MAX_MILLIS: u64 = 15000;

#[get("/sleep/{ms}")]
pub async fn sleep(req: HttpRequest) -> HttpResponse {
    let bad_request =
        HttpResponse::BadRequest().body("Please provide a valid millisecond count 0 - 15000");

    let millis = match req.match_info().get("ms") {
        Some(ms) => {
            if let Ok(ms) = ms.parse() {
                if ms > MAX_MILLIS {
                    return bad_request;
                }

                //can already assume >= 0 if we were able to parse as an unsigned int
                ms
            } else {
                return bad_request;
            }
        }
        None => {
            return not_found().await;
        }
    };

    time::sleep(Duration::from_millis(millis)).await; //non-blocking
    HttpResponse::Ok().finish()
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Not Found</h1>")
}
