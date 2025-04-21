# Kuzzo

A lightweight Pinata.cloud client for Rust that compiles to WebAssembly.

## Features

- Pin files to IPFS via Pinata
- Pin JSON data to IPFS via Pinata
- Unpin content from IPFS
- WASM compatible
- Lightweight and performant

## Installation

Add Kuzzo to your `Cargo.toml`:

```toml
[dependencies]
kuzzo = "0.1.0"
```

## Usage

### Basic Setup

```rust
use kuzzo::PinataClient;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyApp {
    client: PinataClient,
}

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            client: PinataClient::new(api_key, secret_key),
        }
    }
}
```

### Pin a File

```rust
#[wasm_bindgen]
impl MyApp {
    pub async fn pin_file(&self, file_path: String) -> Result<JsValue, JsValue> {
        self.client.pin_file(file_path).await
    }
}

// JavaScript usage:
// const app = new MyApp("your_api_key", "your_secret_key");
// const result = await app.pin_file("path/to/file.txt");
// console.log("File pinned with hash:", result.ipfs_hash);
```

### Pin JSON Data

```rust
#[wasm_bindgen]
impl MyApp {
    pub async fn pin_json(&self, data: JsValue) -> Result<JsValue, JsValue> {
        self.client.pin_json(data).await
    }
}

// JavaScript usage:
// const data = {
//     name: "test",
//     value: 42
// };
// const result = await app.pin_json(data);
// console.log("JSON pinned with hash:", result.ipfs_hash);
```

### Unpin Content

```rust
#[wasm_bindgen]
impl MyApp {
    pub async fn unpin(&self, hash: String) -> Result<(), JsValue> {
        self.client.unpin(hash).await
    }
}

// JavaScript usage:
// await app.unpin("QmHash...");
```

### Complete Example

Here's a complete example showing all features:

```rust
use kuzzo::PinataClient;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub struct MyApp {
    client: PinataClient,
}

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            client: PinataClient::new(api_key, secret_key),
        }
    }

    pub async fn pin_file(&self, file_path: String) -> Result<JsValue, JsValue> {
        self.client.pin_file(file_path).await
    }

    pub async fn pin_json(&self, data: JsValue) -> Result<JsValue, JsValue> {
        self.client.pin_json(data).await
    }

    pub async fn unpin(&self, hash: String) -> Result<(), JsValue> {
        self.client.unpin(hash).await
    }
}

// JavaScript usage:
// const app = new MyApp("your_api_key", "your_secret_key");
//
// // Pin a file
// const fileResult = await app.pin_file("path/to/file.txt");
// console.log("File pinned with hash:", fileResult.ipfs_hash);
//
// // Pin JSON data
// const jsonData = {
//     name: "test",
//     value: 42
// };
// const jsonResult = await app.pin_json(jsonData);
// console.log("JSON pinned with hash:", jsonResult.ipfs_hash);
//
// // Unpin content
// await app.unpin(fileResult.ipfs_hash);
```

## Response Format

All pinning operations return a `PinResponse` object with the following structure:

```typescript
interface PinResponse {
    ipfs_hash: string;    // The IPFS hash of the pinned content
    pin_size: number;     // The size of the pinned content in bytes
    timestamp: string;    // The timestamp when the content was pinned
}
```

## Error Handling

All methods return a `Result` type that can be handled in JavaScript:

```javascript
try {
    const result = await app.pin_file("path/to/file.txt");
    console.log("Success:", result);
} catch (error) {
    console.error("Error:", error);
}
```

## License

MIT 