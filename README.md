# Kuzzo

A lightweight Pinata.cloud client for Rust that compiles to WebAssembly.

## Features

- Pin files to IPFS via Pinata
- Pin JSON data to IPFS via Pinata
- Unpin content from IPFS
- WASM compatible
- Lightweight and performant

## Usage

Add Kuzzo to your `Cargo.toml`:

```toml
[dependencies]
kuzzo = "0.1.0"
```

### Basic Usage

```rust
use kuzzo::PinataClient;

// Initialize the client
let client = PinataClient::new("your_api_key", "your_secret_key");

// Pin a file
let result = client.pin_file("path/to/file.txt").await?;
println!("Pinned file with hash: {}", result.ipfs_hash);

// Pin JSON data
let json_data = serde_json::json!({
    "name": "test",
    "value": 42
});
let result = client.pin_json(&json_data).await?;
println!("Pinned JSON with hash: {}", result.ipfs_hash);

// Unpin content
client.unpin("QmHash...").await?;
```

## WASM Support

Kuzzo is designed to work in WebAssembly environments. When using in a WASM context, make sure to:

1. Use the `wasm-bindgen` feature
2. Handle async operations appropriately in your WASM environment

## License

MIT 