


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

バックエンド側のコードでは、RustとActix-webを使用していることがわかります。この関数`calculate`は、フロントエンドからのPOSTリクエストを受け取って処理します。以下はこの関数の基本的な流れと、その詳細についての説明です。

```rust
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use log::info;

#[derive(Deserialize)]
struct InputData {
    n: i32,
}

async fn calculate(data: web::Json<InputData>) -> Result<HttpResponse> {
    info!("Received calculation request: {:?}", data);

    let n = data.n;
    let result = n * n; // 任意の計算（例えば、nの平方）

    Ok(HttpResponse::Ok().json(result))
}
```

この関数は以下の処理を行います：

1. **データの受け取り**：
   - `web::Json<InputData>`としてフロントエンドから送られてきたJSONデータを受け取ります。
   - `InputData`構造体には、フロントエンドから送信されたJSONデータのフィールド（この場合は`n`）が含まれます。

2. **ログ出力**：
   - `info!`マクロを使用して、受け取ったリクエストデータをログに出力します。

3. **計算処理**：
   - `data.n`から`n`を取得し、任意の計算を行います（例として`n`の平方を計算しています）。

4. **レスポンスの返却**：
   - 計算結果をJSON形式で返します。`HttpResponse::Ok().json(result)`で成功したレスポンスを作成し、計算結果を含めてクライアントに送信します。

これにより、フロントエンドが送信した`n`の値を使用してバックエンドで計算を行い、その結果をJSON形式で返すAPIが構築されます。

全体の流れをまとめると、フロントエンドから送信されたデータをバックエンドで受け取り、処理し、その結果を再びフロントエンドに返すシステムになっています。

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

このコードは、Actix-webを使用してWebアプリケーションを構築するための設定を行う部分です。以下は、コードの各部分についての説明です。

```rust
use actix_web::{App, HttpServer, web, middleware};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### 各部分の説明

1. **`App::new()`**:
   - 新しいActixアプリケーションを作成します。

2. **`wrap(cors)`**:
   - CORS (Cross-Origin Resource Sharing) ミドルウェアを追加します。これは、異なるオリジンからのリクエストを許可するために使用されます。
   - 具体的なCORSの設定はこのコードには含まれていませんが、通常は事前に設定されます。

3. **`wrap(middleware::Logger::default())`**:
   - デフォルトのロガーミドルウェアを追加します。これは、リクエストとレスポンスのログを出力するために使用されます。

4. **`service(fs::Files::new("/", "./static").index_file("index.html"))`**:
   - 静的ファイルを提供するためのサービスを設定します。
   - `"/"`ルートで、`./static`ディレクトリ内のファイルを提供します。
   - `index_file("index.html")`は、ルートにアクセスした場合に`index.html`ファイルを返すように設定します。

5. **`service(web::resource("/calculate").route(web::post().to(calculate)).route(web::get().to(calculate)))`**:
   - `/calculate`エンドポイントに対するリソースを設定します。
   - `web::post().to(calculate)`は、POSTリクエストを受け取った際に`calculate`ハンドラ関数を呼び出します。
   - `web::get().to(calculate)`は、GETリクエストを受け取った際に`calculate`ハンドラ関数を呼び出します。

### まとめ

このコードは、Actix-webアプリケーションを設定して、以下のような機能を提供します：

- CORS対応
- リクエストとレスポンスのログ出力
- 静的ファイルの提供（例：`index.html`）
- `/calculate`エンドポイントへのPOSTおよびGETリクエストの処理

この設定により、フロントエンドのリクエストを受け取り、適切に処理してレスポンスを返すことができます。

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
