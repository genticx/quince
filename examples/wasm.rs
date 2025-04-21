#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use kuzzo::PinataClient;

#[wasm_bindgen]
pub async fn pin_file(api_key: String, secret_key: String, file_path: String) -> Result<JsValue, JsValue> {
    let client = PinataClient::new(api_key, secret_key);
    let result = client.pin_file(file_path).await?;
    Ok(JsValue::from_serde(&result).unwrap())
}

#[wasm_bindgen]
pub async fn pin_json(api_key: String, secret_key: String, data: JsValue) -> Result<JsValue, JsValue> {
    let client = PinataClient::new(api_key, secret_key);
    let result = client.pin_json(data).await?;
    Ok(JsValue::from_serde(&result).unwrap())
}

#[wasm_bindgen]
pub async fn unpin(api_key: String, secret_key: String, hash: String) -> Result<(), JsValue> {
    let client = PinataClient::new(api_key, secret_key);
    client.unpin(hash).await
}

#[wasm_bindgen]
pub fn main() {
    // This is a no-op for WASM, as the functions are exported and can be called from JavaScript
}
