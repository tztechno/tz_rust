

---
failure
```
    <script>
        function sendRequest() {
            const n = document.getElementById('inputN').value;
            fetch('calculate', {
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

---------------------------------------------------------
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
```
---
success
```
    <script>
        async function checkNumber() {
            const number = document.getElementById("number").value;
            const response = await fetch("/check", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ value: parseInt(number) })
            });
            const result = await response.json();
            document.getElementById("result").innerText = `Lucas Number: ${result.result}, Calculation Time: ${result.duration.toFixed(6)} seconds`;
        }
    </script>

---------------------------------------------------------
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
```
---
