use actix_web::{web, Responder};
use serde::Deserialize;
use crate::tables::user::UserDb;

#[derive(Deserialize, Debug)]
struct RegisterBody {
    username: String,
    password: String
}

fn register(body: web::Json<RegisterBody>, user: web::Data<UserDb>) -> impl Responder {
    println!("body: {:?}", body);
	user.insert(&body.username, &body.password);
    format!("New auth")
}

fn login() -> impl Responder {
    format!("New login")
}

fn get_all( user: web::Data<UserDb>) -> impl Responder {
	user.get_all().unwrap_or(String::from("error"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/get_all", web::post().to(get_all))
    ;
}