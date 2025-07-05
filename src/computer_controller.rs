use actix_web::{ web, get, post, Responder, Result };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct Computer {
    id: i64,
    model: String,
    brand: String,
}


#[get("/computer")]
pub async fn index() -> Result<impl Responder> {
    let computer = Computer { 
        id: 1001, 
        model: "MacBook Pro 15-inch 2018".to_string(),
        brand: "Apple".to_string(),
    };

    Ok(web::Json(computer))
}

#[post("/computer")]
pub async fn save(computer: web::Json<Computer>) -> Result<impl Responder> {
    println!("{:?}", &computer);

    Ok(web::Json(computer))
}
