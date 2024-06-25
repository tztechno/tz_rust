###
#
###
2024-06

---
---
---

はい、わかりました。このHTML文書を使用して、バックエンドでLucas数の計算と計算時間の測定を行い、フロントエンドとAjax通信を行うRustのactix-webアプリケーションを作成しましょう。

まず、プロジェクトの構造を以下のように設定します：

```
myapp/
├── Cargo.toml
├── src/
│   └── main.rs
└── static/
    └── index.html
```

1. Cargo.tomlファイルを以下のように更新します：

```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
actix-files = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

2. static/index.htmlファイルに、提供されたHTMLコードを保存します。

3. src/main.rsファイルを以下のように編集します：

```rust
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
```

このコードでは以下のことを行っています：

- `lucas`関数でLucas数を計算します。
- `calculate`関数で、受け取ったnに対してLucas数を計算し、計算時間を測定します。
- 結果をJSON形式で返します。
- HTTPサーバーを設定し、静的ファイル（index.html）の提供と`/calculate`エンドポイントの処理を行います。

4. アプリケーションを実行します：

```
cargo run
```

これで、http://localhost:8080 にアクセスすると、提供されたHTMLページが表示されます。数値を入力して「Calculate」ボタンをクリックすると、バックエンドでLucas数が計算され、結果と計算時間がページに表示されます。

このセットアップについて何か質問がありますか？また、追加の機能や改善点はありますか？

---

Rustでactix-webフレームワークを使用してindex.htmlをホストする基本的な手順を説明します。

1. プロジェクトの作成:
```
cargo new myapp
cd myapp
```

2. Cargo.tomlに依存関係を追加:
```toml
[dependencies]
actix-web = "4"
actix-files = "0.6"
```

3. src/main.rsファイルを編集:

```rust
use actix_web::{App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

4. staticディレクトリを作成し、その中にindex.htmlを配置:
```
mkdir static
echo "<html><body><h1>Hello, Actix!</h1></body></html>" > static/index.html
```

5. サーバーを実行:
```
cargo run
```

これでhttp://localhost:8080 にアクセスするとindex.htmlが表示されます。

このセットアップについて詳しく説明したり、追加の機能を実装したりする必要がありますか？

---
