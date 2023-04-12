use actix_web::{get, Error as ActixError, HttpResponse};

#[get("/heartbeat")]
pub async fn heartbeat() -> Result<HttpResponse, ActixError> {
    Ok(HttpResponse::Ok().body("beep"))
}
