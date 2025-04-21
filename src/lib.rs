mod error;

use serde::{Deserialize, Serialize};
use error::PinataError;
use wasm_bindgen::prelude::*;
use std::path::Path;

const PINATA_API_URL: &str = "https://api.pinata.cloud";

#[derive(Debug, Serialize, Deserialize)]
pub struct PinResponse {
    pub ipfs_hash: String,
    pub pin_size: u64,
    pub timestamp: String,
}

// Native implementation
#[cfg(not(target_arch = "wasm32"))]
pub struct PinataClient {
    api_key: String,
    secret_key: String,
    client: reqwest::blocking::Client,
}

#[cfg(not(target_arch = "wasm32"))]
impl PinataClient {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn pin_file(&self, file_path: &str) -> Result<PinResponse, error::PinataError> {
        let form = reqwest::blocking::multipart::Form::new()
            .file("file", file_path)
            .map_err(|e| error::PinataError::PinFileError(e.to_string()))?;

        let response = self.client
            .post(&format!("{}/pinning/pinFileToIPFS", PINATA_API_URL))
            .header("pinata_api_key", &self.api_key)
            .header("pinata_secret_api_key", &self.secret_key)
            .multipart(form)
            .send()
            .map_err(|e| error::PinataError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(error::PinataError::PinFileError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        response.json()
            .map_err(|e| error::PinataError::DeserializationError(e.to_string()))
    }

    pub fn pin_json<T: Serialize>(&self, data: &T) -> Result<PinResponse, error::PinataError> {
        let response = self.client
            .post(&format!("{}/pinning/pinJSONToIPFS", PINATA_API_URL))
            .header("Content-Type", "application/json")
            .header("pinata_api_key", &self.api_key)
            .header("pinata_secret_api_key", &self.secret_key)
            .json(data)
            .send()
            .map_err(|e| error::PinataError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(error::PinataError::PinJsonError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        response.json()
            .map_err(|e| error::PinataError::DeserializationError(e.to_string()))
    }

    pub fn unpin(&self, hash: &str) -> Result<(), error::PinataError> {
        let response = self.client
            .delete(&format!("{}/pinning/unpin/{}", PINATA_API_URL, hash))
            .header("pinata_api_key", &self.api_key)
            .header("pinata_secret_api_key", &self.secret_key)
            .send()
            .map_err(|e| error::PinataError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(error::PinataError::UnpinError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        Ok(())
    }
}

// WASM implementation
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PinataClient {
    api_key: String,
    secret_key: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PinataClient {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
        }
    }

    #[wasm_bindgen]
    pub async fn pin_file(&self, file_path: String) -> Result<JsValue, JsValue> {
        let form = web_sys::FormData::new()
            .map_err(|e| JsValue::from_str(&format!("Failed to create form: {:?}", e)))?;

        let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&JsValue::from_str(&file_path)))
            .map_err(|e| JsValue::from_str(&format!("Failed to create blob: {:?}", e)))?;

        let file = web_sys::File::new_with_blob_sequence(&js_sys::Array::of1(&blob.into()), "file")
            .map_err(|e| JsValue::from_str(&format!("Failed to create file: {:?}", e)))?;

        form.append_with_blob("file", &file)
            .map_err(|e| JsValue::from_str(&format!("Failed to append file: {:?}", e)))?;

        let mut opts = web_sys::RequestInit::new();
        opts.method("POST");
        opts.body(Some(&form.into()));

        let request = web_sys::Request::new_with_str_and_init(
            &format!("{}/pinning/pinFileToIPFS", PINATA_API_URL),
            &opts,
        ).map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;

        request.headers().set("pinata_api_key", &self.api_key)
            .map_err(|e| JsValue::from_str(&format!("Failed to set API key: {:?}", e)))?;
        request.headers().set("pinata_secret_api_key", &self.secret_key)
            .map_err(|e| JsValue::from_str(&format!("Failed to set secret key: {:?}", e)))?;

        let window = web_sys::window().unwrap();
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to fetch: {:?}", e)))?;

        let response: web_sys::Response = resp_value.dyn_into()
            .map_err(|_| JsValue::from_str("Failed to convert response"))?;

        if !response.ok() {
            return Err(JsValue::from_str(&format!("HTTP error: {}", response.status())));
        }

        wasm_bindgen_futures::JsFuture::from(response.json()?)
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to parse response: {:?}", e)))
    }

    #[wasm_bindgen]
    pub async fn pin_json(&self, data: JsValue) -> Result<JsValue, JsValue> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("POST");
        opts.body(Some(&data));

        let request = web_sys::Request::new_with_str_and_init(
            &format!("{}/pinning/pinJSONToIPFS", PINATA_API_URL),
            &opts,
        ).map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;

        request.headers().set("Content-Type", "application/json")
            .map_err(|e| JsValue::from_str(&format!("Failed to set content type: {:?}", e)))?;
        request.headers().set("pinata_api_key", &self.api_key)
            .map_err(|e| JsValue::from_str(&format!("Failed to set API key: {:?}", e)))?;
        request.headers().set("pinata_secret_api_key", &self.secret_key)
            .map_err(|e| JsValue::from_str(&format!("Failed to set secret key: {:?}", e)))?;

        let window = web_sys::window().unwrap();
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to fetch: {:?}", e)))?;

        let response: web_sys::Response = resp_value.dyn_into()
            .map_err(|_| JsValue::from_str("Failed to convert response"))?;

        if !response.ok() {
            return Err(JsValue::from_str(&format!("HTTP error: {}", response.status())));
        }

        wasm_bindgen_futures::JsFuture::from(response.json()?)
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to parse response: {:?}", e)))
    }

    #[wasm_bindgen]
    pub async fn unpin(&self, hash: String) -> Result<(), JsValue> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("DELETE");

        let request = web_sys::Request::new_with_str_and_init(
            &format!("{}/pinning/unpin/{}", PINATA_API_URL, &hash),
            &opts,
        ).map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;

        request.headers().set("pinata_api_key", &self.api_key)
            .map_err(|e| JsValue::from_str(&format!("Failed to set API key: {:?}", e)))?;
        request.headers().set("pinata_secret_api_key", &self.secret_key)
            .map_err(|e| JsValue::from_str(&format!("Failed to set secret key: {:?}", e)))?;

        let window = web_sys::window().unwrap();
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to fetch: {:?}", e)))?;

        let response: web_sys::Response = resp_value.dyn_into()
            .map_err(|_| JsValue::from_str("Failed to convert response"))?;

        if !response.ok() {
            return Err(JsValue::from_str(&format!("HTTP error: {}", response.status())));
        }

        Ok(())
    }
}
