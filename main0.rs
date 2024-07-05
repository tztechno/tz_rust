use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize)]
struct InputData {
    n: u64,
}

#[derive(Serialize)]
struct OutputData {
    result: u64,
    process_time: f64,
}

fn lucas(n: u64) -> u64 {
    match n {
        0 => 2,
        1 => 1,
        _ => {
            let mut a = 2;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

async fn calculate(data: web::Json<InputData>) -> Result<HttpResponse> {
    let start = Instant::now();
    let result = lucas(data.n);
    let duration = start.elapsed();

    let output = OutputData {
        result,
        process_time: duration.as_secs_f64() * 1000.0,
    };

    Ok(HttpResponse::Ok().json(output))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .service(web::resource("/calculate").route(web::post().to(calculate)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
