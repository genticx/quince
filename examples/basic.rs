#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use kuzzo::PinataClient;

#[wasm_bindgen]
pub async fn run_example(api_key: String, secret_key: String) -> Result<(), JsValue> {
    // Create a new client
    let client = PinataClient::new(api_key, secret_key);

    // Example 1: Pin a file
    web_sys::console::log_1(&JsValue::from_str("Pinning file..."));
    let file_result = client.pin_file("examples/test.txt".to_string()).await?;
    web_sys::console::log_1(&JsValue::from_str(&format!("File pinned with hash: {}", file_result.ipfs_hash)));

    // Example 2: Pin JSON data
    web_sys::console::log_1(&JsValue::from_str("Pinning JSON..."));
    let json_data = serde_json::json!({
        "name": "test",
        "value": 42
    });
    let json_result = client.pin_json(JsValue::from_serde(&json_data).unwrap()).await?;
    web_sys::console::log_1(&JsValue::from_str(&format!("JSON pinned with hash: {}", json_result.ipfs_hash)));

    // Example 3: Unpin content
    web_sys::console::log_1(&JsValue::from_str("Unpinning content..."));
    client.unpin(file_result.ipfs_hash).await?;
    web_sys::console::log_1(&JsValue::from_str("Content unpinned successfully"));

    Ok(())
}

#[wasm_bindgen]
pub fn main() {
    // This is a no-op for WASM, as the functions are exported and can be called from JavaScript
}
