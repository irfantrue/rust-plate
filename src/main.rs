use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Home {
    status: bool,
    message: String,
    result: String,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let obj = Home {
        status: true,
        message: "OK".to_string(),
        result: "Welcome RUST API V1".to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
