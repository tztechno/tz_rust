

---

RustでAjax通信を行うには、一般的にWebAssembly (Wasm)を使用してRustコードをブラウザで動かし、JavaScriptと連携させる方法が取られます。以下に、基本的な手順を説明します。

### 1. Rustプロジェクトのセットアップ

最初に、`wasm-pack`をインストールします。`wasm-pack`は、RustのWebAssemblyプロジェクトをビルドおよびパッケージ化するためのツールです。

```bash
cargo install wasm-pack
```

次に、Rustのプロジェクトを作成します。

```bash
cargo new my-wasm-project --lib
cd my-wasm-project
```

### 2. Rustコードの作成

次に、`src/lib.rs`ファイルを編集して、Wasmバインディングを作成します。

```rust
use wasm_bindgen::prelude::*;
use web_sys::XmlHttpRequest;

#[wasm_bindgen]
pub fn make_request(url: &str) -> Result<JsValue, JsValue> {
    let xhr = XmlHttpRequest::new().unwrap();
    xhr.open("GET", url).unwrap();

    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let response = xhr.response_text().unwrap().unwrap();
        web_sys::console::log_1(&response.into());
    }) as Box<dyn FnMut(_)>);

    xhr.set_onload(Some(onload.as_ref().unchecked_ref()));
    xhr.send().unwrap();

    onload.forget();

    Ok(JsValue::NULL)
}
```

### 3. `wasm-pack`を使用してビルド

次に、`wasm-pack`を使用してプロジェクトをビルドします。

```bash
wasm-pack build --target web
```

### 4. JavaScriptでRust関数を呼び出す

次に、HTMLファイルとJavaScriptファイルを作成し、RustのWasm関数を呼び出します。

#### index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Wasm Ajax</title>
</head>
<body>
    <script type="module">
        import init, { make_request } from "./pkg/my_wasm_project.js";

        async function run() {
            await init();
            make_request("https://api.example.com/data");
        }

        run();
    </script>
</body>
</html>
```

### 5. ローカルサーバーを起動

HTMLファイルをブラウザでテストするために、簡単なHTTPサーバーを起動します。例えば、Pythonを使用する場合：

```bash
python -m http.server
```

ブラウザで`http://localhost:8000`を開き、Ajaxリクエストが正常に行われることを確認します。

これで、RustとWasmを使用した基本的なAjax通信が実現できます。より高度なエラーハンドリングやリクエストの種類（POSTなど）の対応は、この基本構造を拡張して行えます。


---
```

```
```

```
---
## make_request
```
    <script type="module">
        import init, { make_request } from "./pkg/my_wasm_project.js";
        async function run() {
            await init();
            make_request("https://api.example.com/data");
        }
        run();
    </script>

```
```

pub fn make_request(url: &str) -> Result<JsValue, JsValue> {
    let xhr = XmlHttpRequest::new().unwrap();
    xhr.open("GET", url).unwrap();

    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let response = xhr.response_text().unwrap().unwrap();
        web_sys::console::log_1(&response.into());
    }) as Box<dyn FnMut(_)>);

    xhr.set_onload(Some(onload.as_ref().unchecked_ref()));
    xhr.send().unwrap();

    onload.forget();

    Ok(JsValue::NULL)
}

```
---
