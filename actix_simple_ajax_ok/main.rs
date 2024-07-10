use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Number {
    value: i32,
}

async fn index() -> impl Responder {
    NamedFile::open("static/index.html")
}

async fn check_number(num: web::Json<Number>) -> impl Responder {
    let result = if num.value % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/check", web::post().to(check_number))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
