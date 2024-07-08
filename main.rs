use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize)]
struct InputData {
    n: u32,
}

#[derive(Serialize)]
struct OutputData {
    result: u64,
    process_time: u128,
}

fn calculate_lucas(n: u32) -> u64 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 2;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

async fn calculate(data: web::Json<InputData>) -> impl Responder {
    let start_time = Instant::now();
    let result = calculate_lucas(data.n);
    let process_time = start_time.elapsed().as_nanos();

    HttpResponse::Ok().json(OutputData {
        result,
        process_time,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .route("/calculate", web::post().to(calculate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}