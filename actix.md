###
#
###
2024-06

---
---
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
