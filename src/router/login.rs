use actix_web::{web, Responder};

fn register() -> impl Responder {
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