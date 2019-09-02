use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RegisterBody {
    username: String,
    password: String
}

fn register(body: web::Json<RegisterBody>) -> impl Responder {
    println!("body: {:?}", body);
    format!("New auth")
}

fn login() -> impl Responder {
    format!("New login")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
    ;
}