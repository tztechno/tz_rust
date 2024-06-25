###
#
###
2024-06

---
---
---
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
