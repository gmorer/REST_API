use actix_web::{web, App, middleware, HttpServer, HttpResponse};
use postgres::{Client, NoTls};
use dotenv::dotenv;
use std::env;

mod router;
use router::{ login };

mod tables;
use tables::user::User;

fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let db_host = env::var("DATABASE_HOST").expect("no database host");
    let db_user = env::var("DATABASE_USER").expect("no database user");
    let db_params = format!("host={} user={}", db_host, db_user);
    let mut db_client = Client::connect(&db_params, NoTls).expect("cannot connect to the database");
    let user = User::builder(&mut db_client);

    HttpServer::new(|| App::new()
        .wrap(middleware::NormalizePath)
        .service(
            web::scope("/").configure(login::config)
            // .data(user) TODO pass user to the login route 
        )
        .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
    )
    .bind("127.0.0.1:8080")?
    .run()
}