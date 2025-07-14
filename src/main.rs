use actix_web::{ web, App, HttpServer };

use crate::hello_world_controller::{hello, echo, hello_world, manual_hello};
use crate::computer_controller::{index, save};
use crate::health_check::{admin_health_check, admin_env_route};


mod hello_world_controller;
mod user_controller;
mod computer_controller;
mod health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting app!");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/user")
                    .configure(user_controller::route_config)
            )
            .service(index)
            .service(save)
            .service(hello)
            .service(echo)
            .service(hello_world)
            .service(admin_health_check)
            .service(admin_env_route)
            .route("/hey", web::get().to(manual_hello))
        })
    .bind("0.0.0.0:8080").unwrap()
    .run()
    .await
}
