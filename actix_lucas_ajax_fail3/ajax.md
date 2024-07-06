

---
html: fetch('/calculate'
```
    <script>
        function sendRequest() {
            const n = document.getElementById('inputN').value;
            fetch('/calculate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ n: parseInt(n) })
            })
                .then(response => {
                    if (!response.ok) {
                        throw new Error('Network response was not ok');
                    }
                    return response.json();
                })
                .then(data => {
                    document.getElementById('result').innerText = `Lucas Number L${n} = ${data.result}`;
                    document.getElementById('time').innerText = `Time: ${(data.process_time / 1e9).toFixed(3)} sec`;
                })
                .catch(error => {
                    console.error('Error:', error);
                    document.getElementById('result').innerText = 'An error occurred. Please try again.';
                    document.getElementById('time').innerText = '';
                });
        }
    </script>
```
---
rs: async fn calculate
```
async fn calculate(data: web::Json<InputData>) -> Result<HttpResponse> {
    info!("Received calculation request: {:?}", data);
    let start = Instant::now();
    
    let result = lucas(data.n);
    let duration = start.elapsed();

    let output = OutputData {
        result,
        process_time: duration.as_secs_f64() * 1000.0,
    };

    info!("Calculation completed. Result: {}, Time: {}ms", result, output.process_time);
    Ok(HttpResponse::Ok().json(output))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::permissive();
            
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .service(
                web::resource("/calculate")
                    .route(web::post().to(calculate))
                    .route(web::get().to(calculate))
            )
    })

```
---
