use actix_web::{get, Responder, HttpResponse, web, Result};
use std::env;
use std::collections::HashMap;

#[get("/admin/health")]
pub async fn admin_health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[get("/admin/env")]
pub async fn admin_env_route() -> Result<impl Responder> {
    let vars = env::vars();
    let mut env_variables: HashMap<String, String> = HashMap::new();

    for (k, v) in vars {
        env_variables.insert(k, v);
    }

    Ok(web::Json(env_variables))
}