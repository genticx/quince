#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use kuzzo::PinataClient;

#[wasm_bindgen]
pub struct Client {
    client: PinataClient,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            client: PinataClient::new(api_key, secret_key),
        }
    }

    #[wasm_bindgen]
    pub async fn pin_file(&self, file_path: String) -> Result<JsValue, JsValue> {
        let result = self.client.pin_file(file_path).await?;
        Ok(JsValue::from_serde(&result).unwrap())
    }

    #[wasm_bindgen]
    pub async fn pin_json(&self, data: JsValue) -> Result<JsValue, JsValue> {
        let result = self.client.pin_json(data).await?;
        Ok(JsValue::from_serde(&result).unwrap())
    }

    #[wasm_bindgen]
    pub async fn unpin(&self, hash: String) -> Result<(), JsValue> {
        self.client.unpin(hash).await
    }
}

#[wasm_bindgen]
pub fn main() {
    // This is a no-op for WASM, as the functions are exported and can be called from JavaScript
}
