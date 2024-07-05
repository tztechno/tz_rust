


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

このコードは、JavaScriptの`fetch` APIを使用して、サーバーに対してPOSTリクエストを送信しています。具体的には、以下の処理を行っています。

1. `'calculate'`エンドポイントに対してリクエストを送信。
2. HTTPメソッドは`POST`。
3. リクエストヘッダーには、`Content-Type`として`'application/json'`を設定。
4. `body`には、`n`という変数をJSON形式の文字列に変換して送信。

ここでの`n`は、整数に変換された値（`parseInt(n)`）が送られています。サーバー側では、この`n`の値を使用して計算を行うか、何らかの処理を行うことが予想されます。

例：

```javascript
let n = 10; // 送信する値
fetch('calculate', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({ n: parseInt(n) })
})
.then(response => response.json())
.then(data => {
    console.log(data); // サーバーからの応答を処理
})
.catch(error => {
    console.error('Error:', error);
});
```

このコードでは、`n`の値をサーバーに送信し、サーバーからの応答を処理しています。


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
