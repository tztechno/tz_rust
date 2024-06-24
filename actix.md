###
#
###
2024-06

---
---


RustとActixを使って、同様のLucas数計算アプリケーションを構築する方法を説明します。以下の手順に従ってください。

### 手順概要

1. **プロジェクトの作成**
2. **依存関係の追加**
3. **静的ファイルの設定**
4. **ルーティングの設定**
5. **Lucas数の計算と時間計測の実装**
6. **フロントエンドとの連携**

### 手順詳細

#### 1. プロジェクトの作成

まず、新しいRustプロジェクトを作成します。

```sh
cargo new myapp
cd myapp
```

#### 2. 依存関係の追加

`Cargo.toml`ファイルを開き、必要な依存関係を追加します。`actix-web` と `serde` を使用します。

```toml
[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### 3. 静的ファイルの設定

`src/main.rs` ファイルを次のように設定します。

```rust
use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::time::Instant;

#[derive(Deserialize)]
struct CalculationRequest {
    n: u32,
}

async fn calculate_lucas(req: web::Json<CalculationRequest>) -> impl Responder {
    let n = req.n;
    let start_time = Instant::now();
    let result = calculate_lucas_number(n);
    let duration = start_time.elapsed();

    web::Json(serde_json::json!({
        "result": result,
        "process_time": duration.as_millis()
    }))
}

fn calculate_lucas_number(n: u32) -> u64 {
    match n {
        0 => 2,
        1 => 1,
        _ => calculate_lucas_number(n - 1) + calculate_lucas_number(n - 2),
    }
}

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/calculate", web::post().to(calculate_lucas))
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

#### 4. フロントエンドの準備

`static` フォルダをプロジェクトのルートに作成し、`index.html` をその中に配置します。

```html
<!DOCTYPE html>
<html>

<head>
    <title>Lucas Number Calculator</title>
</head>

<body>
    <p>AJAX Lucas Number</p>
    <input type="number" id="inputN" placeholder="Enter a number">
    <button onclick="sendRequest()">Calculate</button>
    <div id="result"></div>
    <div id="time"></div>

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
                .then(response => response.json())
                .then(data => {
                    document.getElementById('result').innerText = `Lucas Number L${n} = ${data.result}`;
                    document.getElementById('time').innerText = `Time: ${(data.process_time / 1000).toFixed(6)} sec`;
                })
                .catch(error => {
                    console.error('Error:', error);
                });
        }
    </script>
</body>

</html>
```

#### 5. アプリケーションのビルドと実行

次に、アプリケーションをビルドして実行します。

```sh
cargo run
```

ブラウザで `http://127.0.0.1:8080` にアクセスし、数値を入力して "Calculate" ボタンをクリックすると、計算された Lucas 数とその計算時間が表示されるはずです。

これで、RustとActixを使用してフロントエンドとバックエンドが連携するLucas数計算アプリケーションが完成しました。何か他に質問があれば教えてください。

---
myappにactix-webをインストールして、HTMLをホストする方法について説明します。

### 1. Rustのセットアップ

まず、RustとCargoがインストールされていることを確認してください。インストールされていない場合は、[Rustの公式ウェブサイト](https://www.rust-lang.org/tools/install)からインストールできます。

### 2. 新しいRustプロジェクトの作成

次に、新しいRustプロジェクトを作成します。

```bash
cargo new myapp
cd myapp
```

### 3. `actix-web`の追加

`actix-web`をプロジェクトに追加します。`Cargo.toml`ファイルを開き、以下の依存関係を追加します。

```toml
[dependencies]
actix-web = "4.0.0"
```

### 4. HTMLファイルの用意

プロジェクトのルートディレクトリに、`index.html`などのHTMLファイルを作成します。

```html
<!-- index.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Actix Web App</title>
</head>
<body>
    <h1>Hello, Actix-Web!</h1>
    <p>This is a simple HTML page served by Actix-Web.</p>
</body>
</html>
```

### 5. Actix Webアプリケーションの作成

`src/main.rs`ファイルを編集して、Actix Webアプリケーションを作成します。

```rust
// src/main.rs
use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;

async fn index() -> HttpResponse {
    let html_content = fs::read_to_string("index.html")
        .expect("Failed to read index.html");
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### 6. アプリケーションの実行

最後に、アプリケーションをビルドして実行します。

```bash
cargo run
```

デフォルトでは、アプリケーションは `http://127.0.0.1:8080` で実行されます。ブラウザでこのURLにアクセスすると、`index.html`が表示されるはずです。

以上で、Actix Webを使用してHTMLをホストする基本的な方法が完成します。特定のファイルやディレクトリを公開したり、より複雑なルーティングや処理を追加したりすることもできますが、これで基本的な構造を理解できるはずです。
---
