


### latest

---
```
            fetch('calculate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ n: parseInt(n) })
            })
```
---
```
async fn calculate(data: web::Json<InputData>) -> Result<HttpResponse> {
    info!("Received calculation request: {:?}", data);
```
---
```
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .service(
                web::resource("/calculate")
                    .route(web::post().to(calculate))
                    .route(web::get().to(calculate))
            )
```
---
```
                .then(data => {
                    document.getElementById('result').innerText = `Lucas Number L${n} = ${data.result}`;
                    document.getElementById('time').innerText = `Time: ${(data.process_time / 1e9).toFixed(3)} sec`;
                })
```
---
```

```
---
```

```
---
```

```
---
