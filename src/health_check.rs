use actix_web::{get, Responder, HttpResponse};

#[get("/admin/health")]
pub async fn admin_health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
