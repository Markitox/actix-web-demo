use std::sync::atomic::{AtomicUsize, Ordering};

use actix_web::{get, HttpResponse, post, Responder};

static GLOBAL_REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/hello")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/helloWorld")]
pub async fn hello_world() -> impl Responder {
    let request_number = GLOBAL_REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);

    format!("Request # {}", request_number)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}
