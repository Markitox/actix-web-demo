use actix_web::{get, Responder, HttpResponse};
use rand::Rng;

#[get("/admin/health")]
pub async fn admin_health_check() -> impl Responder {
    let mut rng = rand::rng();
    let bool_rand = rng.random_bool(0.5);

    if bool_rand {
        HttpResponse::Ok().body("Ok")
    } else {
        HttpResponse::InternalServerError().finish()
    }

}
