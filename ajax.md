
---
---

RustとWebAssembly（Wasm）を使用して、フロントエンドで入力された値をAjax通信でPOSTし、その計算結果のレスポンスをフロントエンドで受け取る手順を説明します。これには、RustでWasmを使用し、JavaScriptとの連携を行う方法を含みます。

### 1. Rustプロジェクトのセットアップ

`wasm-pack`がインストールされていることを前提とします。

### 2. Rustコードの作成

RustでPOSTリクエストを送信し、レスポンスを処理するコードを作成します。

#### `src/lib.rs`

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{XmlHttpRequest, XmlHttpRequestResponseType, Event, ProgressEvent};

#[wasm_bindgen]
pub fn make_post_request(url: &str, data: &str) -> Result<JsValue, JsValue> {
    let xhr = XmlHttpRequest::new().unwrap();
    xhr.open("POST", url).unwrap();
    xhr.set_request_header("Content-Type", "application/json").unwrap();

    let onload = Closure::wrap(Box::new(move |_: ProgressEvent| {
        let response = xhr.response_text().unwrap().unwrap();
        web_sys::console::log_1(&response.into());
        // Handle the response here. For example, update a DOM element with the response.
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.get_element_by_id("result").unwrap();
        element.set_inner_html(&response);
    }) as Box<dyn FnMut(_)>);

    let onerror = Closure::wrap(Box::new(move |_: ProgressEvent| {
        web_sys::console::log_1(&"An error occurred".into());
    }) as Box<dyn FnMut(_)>);

    xhr.set_onload(Some(onload.as_ref().unchecked_ref()));
    xhr.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    xhr.send_with_str(data).unwrap();

    onload.forget();
    onerror.forget();

    Ok(JsValue::NULL)
}
```

### 3. `wasm-pack`を使用してビルド

次に、`wasm-pack`を使用してプロジェクトをビルドします。

```bash
wasm-pack build --target web
```

### 4. HTMLとJavaScriptの設定

HTMLファイルとJavaScriptファイルを作成し、フロントエンドから値を取得し、RustのWasm関数を呼び出します。

#### `index.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Wasm Ajax POST</title>
</head>
<body>
    <input type="text" id="input" placeholder="Enter value" />
    <button id="send">Send</button>
    <div id="result"></div>

    <script type="module">
        import init, { make_post_request } from "./pkg/my_wasm_project.js";

        async function run() {
            await init();

            document.getElementById("send").addEventListener("click", () => {
                const input = document.getElementById("input").value;
                const data = JSON.stringify({ value: input });
                make_post_request("https://api.example.com/compute", data);
            });
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

ブラウザで`http://localhost:8000`を開き、入力した値をPOSTリクエストとして送信し、そのレスポンスをフロントエンドで受け取り、表示することを確認します。

これで、RustとWasmを使用して、フロントエンドで入力された値をAjax通信でPOSTし、その計算結果のレスポンスをフロントエンドで受け取ることができます。

---
---

`make_post_request`は、RustのWasmコードで定義した関数です。この関数をJavaScriptから呼び出して、POSTリクエストを行います。`fetch`を使用しない理由としては、Rustで`XmlHttpRequest`を使用した場合の例を示しているからです。`fetch`はよりモダンな方法であり、同様の機能を実現できます。

### `fetch`を使用する場合の方法

`fetch` APIを使用して同様の機能を実現する方法も紹介します。`fetch`を使った場合、Rust側では単にJavaScriptの関数を呼び出し、POSTリクエストを行います。

#### 1. Rust側でJavaScript関数を呼び出す

Rust側でJavaScriptの`fetch`関数を呼び出すようにします。

##### `src/lib.rs`

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[wasm_bindgen]
pub async fn make_post_request(url: &str, data: &str) -> Result<JsValue, JsValue> {
    let window = window().expect("no global `window` exists");
    let fetch = window.fetch_with_str(url);

    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(data)));
    opts.headers(&web_sys::Headers::new().unwrap().append("Content-Type", "application/json").unwrap());

    let request = web_sys::Request::new_with_str_and_init(url, &opts)?;

    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: web_sys::Response = response.dyn_into().unwrap();

    let text = JsFuture::from(response.text()?).await?;
    web_sys::console::log_1(&text);

    // Update the DOM with the response
    let document = window.document().expect("should have a document on window");
    let element = document.get_element_by_id("result").expect("should have #result on the page");
    element.set_inner_html(&text.as_string().unwrap());

    Ok(JsValue::NULL)
}
```

### 2. HTMLとJavaScriptの設定

HTMLファイルとJavaScriptファイルを作成し、フロントエンドから値を取得し、RustのWasm関数を呼び出します。

#### `index.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Wasm Ajax POST with fetch</title>
</head>
<body>
    <input type="text" id="input" placeholder="Enter value" />
    <button id="send">Send</button>
    <div id="result"></div>

    <script type="module">
        import init, { make_post_request } from "./pkg/my_wasm_project.js";

        async function run() {
            await init();

            document.getElementById("send").addEventListener("click", async () => {
                const input = document.getElementById("input").value;
                const data = JSON.stringify({ value: input });
                await make_post_request("https://api.example.com/compute", data);
            });
        }

        run();
    </script>
</body>
</html>
```

### 3. `wasm-pack`を使用してビルド

次に、`wasm-pack`を使用してプロジェクトをビルドします。

```bash
wasm-pack build --target web
```

### 4. ローカルサーバーを起動

HTMLファイルをブラウザでテストするために、簡単なHTTPサーバーを起動します。例えば、Pythonを使用する場合：

```bash
python -m http.server
```

ブラウザで`http://localhost:8000`を開き、入力した値をPOSTリクエストとして送信し、そのレスポンスをフロントエンドで受け取り、表示することを確認します。

これで、`fetch` APIを使用してRustとWasmを使用したPOSTリクエストの例を実現できます。`fetch`を使用することで、よりモダンで柔軟な方法でAjax通信を行うことができます。

---
---
