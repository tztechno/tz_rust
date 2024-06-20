use actix_files as fs;
use actix_web::{post, web, App, HttpServer, HttpResponse, Result, Error};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct InputData {
    n: u64,
}

#[derive(Serialize)]
struct OutputData {
    result: u64,
    process_time: u128,
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

#[post("/calculate")]
async fn calculate(data: web::Json<InputData>) -> Result<HttpResponse, Error> {
    println!("Received request: {:?}", data);

    let start = Instant::now();
    let result = lucas(data.n);
    let duration = start.elapsed().as_millis();

    let response = HttpResponse::Ok().json(OutputData {
        result,
        process_time: duration,
    });

    println!("Sending response: {:?}", response);

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                println!("JSON deserialization error: {:?}", err);
                actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
            }))
            .service(calculate)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
