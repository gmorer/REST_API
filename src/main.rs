#[macro_use]
extern crate derive_more;
use actix_web::{web, App, middleware, HttpServer, HttpResponse};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use dotenv::dotenv;
use std::env;

mod router;
use router::{ login };

pub mod tables;
use tables::user::UserDb;

fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let db_host = env::var("DATABASE_HOST").expect("no database host");
    let db_user = env::var("DATABASE_USER").expect("no database user");
    let db_params = format!("host={} user={}", db_host, db_user);
	let manager = PostgresConnectionManager::new(
        db_params.parse().unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).expect("Cannot create database pool");
	{
    	UserDb::builder(&mut pool.clone().get().expect("Cannot get the first clone"));
	}
    HttpServer::new(move || App::new()
        .wrap(middleware::NormalizePath)
		.data(UserDb::new(pool.clone()))
        .service(
            web::scope("/").configure(login::config)
            // .data(user) TODO pass user to the login route 
        )
        .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
    )
    .bind("127.0.0.1:8080")?
    .run()
}