use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const PINATA_API_URL: &str = "https://api.pinata.cloud";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PinResponse {
    #[serde(rename = "IpfsHash")]
    pub ipfs_hash: String,
    #[serde(rename = "PinSize")]
    pub pin_size: u64,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfFiles")]
    pub number_of_files: u64,
    #[serde(rename = "MimeType")]
    pub mime_type: Option<serde_json::Value>,
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,
    #[serde(rename = "Keyvalues")]
    pub keyvalues: Option<serde_json::Value>,
}

#[wasm_bindgen]
pub struct PinataClient {
    api_key: String,
    secret_key: String,
}

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

        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        let form_js: JsValue = form.into();
        opts.set_body(&form_js);

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
        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&data);

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
    pub async fn pin_file_blob(&self, blob: web_sys::Blob) -> Result<JsValue, JsValue> {
        let form = web_sys::FormData::new()
            .map_err(|e| JsValue::from_str(&format!("Failed to create form: {:?}", e)))?;

        form.append_with_blob("file", &blob)
            .map_err(|e| JsValue::from_str(&format!("Failed to append blob: {:?}", e)))?;

        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        let form_js: JsValue = form.into();
        opts.set_body(&form_js);

        let request = web_sys::Request::new_with_str_and_init(
            &format!("{}/pinning/pinFileToIPFS", PINATA_API_URL),
            &opts,
        ).map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;

        request.headers().set("pinata_api_key", &self.api_key)?;
        request.headers().set("pinata_secret_api_key", &self.secret_key)?;

        let window = web_sys::window().unwrap();
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await?;

        let response: web_sys::Response = resp_value.dyn_into()?;

        if !response.ok() {
            return Err(JsValue::from_str(&format!("HTTP error: {}", response.status())));
        }

        wasm_bindgen_futures::JsFuture::from(response.json()?)
            .await
    }

    #[wasm_bindgen]
    pub async fn unpin(&self, hash: String) -> Result<(), JsValue> {
        let opts = web_sys::RequestInit::new();
        opts.set_method("DELETE");

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
