# Quince

A lightweight pinata.cloud wasm client in Rust.

## Features

- Pin files to IPFS via Pinata
- Pin JSON data to IPFS via Pinata
- Unpin content from IPFS
- WASM compatible
- Lightweight and performant

## Installation

Add quince to your `Cargo.toml`:

```toml
[dependencies]
quince = "0.1.0"
```

## Usage

### Basic Setup

```rust
use quince::PinataClient;

// Initialize the client
let client = PinataClient::new("your_api_key", "your_secret_key");
```

### Pin a File

```rust
use quince::PinataClient;

async fn pin_file_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = PinataClient::new("your_api_key", "your_secret_key");
    let result = client.pin_file("path/to/file.txt").await?;
    println!("File pinned with hash: {}", result.ipfs_hash);
    Ok(())
}
```

### Pin JSON Data

```rust
use quince::PinataClient;
use serde_json::json;

async fn pin_json_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = PinataClient::new("your_api_key", "your_secret_key");
    let data = json!({
        "name": "test",
        "value": 42
    });
    let result = client.pin_json(&data).await?;
    println!("JSON pinned with hash: {}", result.ipfs_hash);
    Ok(())
}
```

### Unpin Content

```rust
use quince::PinataClient;

async fn unpin_example() -> Result<(), Box<dyn std::error::Error>> {
    let client = PinataClient::new("your_api_key", "your_secret_key");
    client.unpin("QmHash...").await?;
    println!("Content unpinned successfully");
    Ok(())
}
```

### Usage in Web Frameworks

#### Leptos Example

```rust
use leptos::*;
use quince::PinataClient;

#[component]
pub fn PinButton() -> impl IntoView {
    let client = PinataClient::new("your_api_key", "your_secret_key");
    
    let pin_file = create_action(move |file_path: &String| {
        let client = client.clone();
        async move {
            let result = client.pin_file(file_path.clone()).await?;
            Ok::<_, Box<dyn std::error::Error>>(result.ipfs_hash)
        }
    });

    view! {
        <button on:click=move |_| {
            pin_file.dispatch("path/to/file.txt".to_string());
        }>
            "Pin File"
        </button>
    }
}
```

#### Yew Example

```rust
use yew::prelude::*;
use quince::PinataClient;

#[function_component(PinButton)]
pub fn pin_button() -> Html {
    let client = PinataClient::new("your_api_key", "your_secret_key");
    
    let onclick = Callback::from(move |_| {
        let client = client.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(result) = client.pin_file("path/to/file.txt").await {
                log::info!("File pinned with hash: {}", result.ipfs_hash);
            }
        });
    });

    html! {
        <button {onclick}>
            { "Pin File" }
        </button>
    }
}
```

## Response Format

All pinning operations return a `PinResponse` struct with the following fields:

```rust
pub struct PinResponse {
    pub ipfs_hash: String,    // The IPFS hash of the pinned content
    pub pin_size: u64,        // The size of the pinned content in bytes
    pub timestamp: String,    // The timestamp when the content was pinned
}
```

## Error Handling

All methods return a `Result` type that can be handled using Rust's error handling:

```rust
use quince::PinataClient;

async fn handle_errors() -> Result<(), Box<dyn std::error::Error>> {
    let client = PinataClient::new("your_api_key", "your_secret_key");
    
    match client.pin_file("path/to/file.txt").await {
        Ok(result) => println!("Success: {}", result.ipfs_hash),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    Ok(())
}
```
