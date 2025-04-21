mod error;
mod platform;

use serde::{Deserialize, Serialize};
use error::{PinataError, Result};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct PinResponse {
    pub ipfs_hash: String,
    pub pin_size: u64,
    pub timestamp: String,
}

pub struct PinataClient {
    api_key: String,
    secret_key: String,
}

impl PinataClient {
    pub fn new(api_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    pub async fn pin_file(&self, file_path: &str) -> Result<PinResponse> {
        platform::pin_file(self, file_path).await
    }

    pub async fn pin_json<T: Serialize>(&self, data: &T) -> Result<PinResponse> {
        platform::pin_json(self, data).await
    }

    pub async fn unpin(&self, hash: &str) -> Result<()> {
        platform::unpin(self, hash).await
    }
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
pub struct WasmPinataClient(PinataClient);

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
impl WasmPinataClient {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self(PinataClient::new(api_key, secret_key))
    }

    #[wasm_bindgen]
    pub async fn pin_file(&self, file_path: String) -> Result<PinResponse, JsValue> {
        self.0.pin_file(&file_path)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub async fn pin_json(&self, data: JsValue) -> Result<PinResponse, JsValue> {
        let json_data: serde_json::Value = data.into_serde()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.0.pin_json(&json_data)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub async fn unpin(&self, hash: String) -> Result<(), JsValue> {
        self.0.unpin(&hash)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
} 