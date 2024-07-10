use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Number {
    value: usize,
}

async fn index() -> impl Responder {
    NamedFile::open("static/index.html")
}

fn lucas_number(n: usize) -> usize {
    match n {
        0 => 2,
        1 => 1,
        _ => {
            let mut a = 2;
            let mut b = 1;
            for _ in 2..=n {
                let temp = b;
                b = a + b;
                a = temp;
            }
            b
        }
    }
}

async fn check_number(num: web::Json<Number>) -> impl Responder {
    let result = lucas_number(num.value);
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
