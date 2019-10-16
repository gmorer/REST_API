#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate diesel;
use actix_web::{web, App, middleware, HttpServer, HttpResponse};
use diesel::r2d2::{ ConnectionManager, Pool };
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod router;
use router::{ login };

pub mod tables;
use tables::user::UserDb;

fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let db_host = env::var("DATABASE_URL").expect("no database host");
	let manager: ConnectionManager<PgConnection> = ConnectionManager::new(db_host);
    let pool = Pool::new(manager).expect("Cannot create database pool");
	{
    	// UserDb::builder(&mut pool.clone().get().expect("Cannot get the first clone"));
	}
	println!("yeah...");
    HttpServer::new(move || App::new()
        .wrap(middleware::NormalizePath)
		.data(UserDb::new(pool.clone()))
        .service(
            web::scope("/").configure(login::config)
        )
        .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
    )
    .bind("127.0.0.1:8080")?
    .run()
}