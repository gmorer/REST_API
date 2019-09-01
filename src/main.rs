use actix_web::{web, App, middleware, HttpServer, HttpResponse};

mod router;
use router::login;

// fn index(info: web::Path<(String, u32)>) -> impl Responder {
//     format!("Hello {}! id:{}", info.0, info.1)
// }

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .wrap(middleware::NormalizePath)
        .service(
            web::scope("/").configure(login::config)
            // web::resource("/{name}/{id}/index.html").to(index))
        )
        // .default_service(|| HttpResponse::MethodNotAllowed)
    )
    .bind("127.0.0.1:8080")?
    .run()
}