
cd myapp

http://localhost:8080

---
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
```
---
```
async fn calculate(data: web::Json<InputData>) -> impl Responder {
    let start_time = Instant::now();
    let result = calculate_lucas(data.n);
    let process_time = start_time.elapsed().as_nanos();

    HttpResponse::Ok().json(OutputData {
        result,
        process_time,
    })
}
```
---