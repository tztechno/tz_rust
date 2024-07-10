use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::time::Instant;

#[derive(Deserialize)]
struct Number {
    value: usize,
}

#[derive(serde::Serialize)]
struct LucasResult {
    result: usize,
    duration: f64,
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
    let start = Instant::now();
    let result = lucas_number(num.value);
    let duration = start.elapsed().as_secs_f64();

    let response = LucasResult {
        result,
        duration,
    };

    HttpResponse::Ok().json(response)
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
