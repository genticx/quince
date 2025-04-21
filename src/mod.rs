mod wasm;

pub use wasm::{pin_file, pin_json, unpin};

use super::{PinataClient, PinResponse, Result, PinataError};
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

const PINATA_API_URL: &str = "https://api.pinata.cloud";

pub async fn pin_file(client: &PinataClient, file_path: &str) -> Result<PinResponse> {
    let file = web_sys::File::new_with_str(file_path, "")
        .map_err(|e| PinataError::PinFileError(format!("Failed to create file: {:?}", e)))?;

    let form_data = web_sys::FormData::new()
        .map_err(|e| PinataError::PinFileError(format!("Failed to create form data: {:?}", e)))?;
    
    form_data.append_with_blob("file", &file)
        .map_err(|e| PinataError::PinFileError(format!("Failed to append file: {:?}", e)))?;

    let mut headers = Headers::new()
        .map_err(|e| PinataError::NetworkError(format!("Failed to create headers: {:?}", e)))?;
    
    headers.append("pinata_api_key", &client.api_key)
        .map_err(|e| PinataError::NetworkError(format!("Failed to set API key: {:?}", e)))?;
    
    headers.append("pinata_secret_api_key", &client.secret_key)
        .map_err(|e| PinataError::NetworkError(format!("Failed to set secret key: {:?}", e)))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.headers(&headers);
    opts.body(Some(&form_data.into()));

    let request = Request::new_with_str_and_init(
        &format!("{}/pinning/pinFileToIPFS", PINATA_API_URL),
        &opts,
    ).map_err(|e| PinataError::NetworkError(format!("Failed to create request: {:?}", e)))?;

    let window = web_sys::window().unwrap();
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| PinataError::NetworkError(format!("Failed to fetch: {:?}", e)))?;

    let response: Response = response.dyn_into()
        .map_err(|e| PinataError::NetworkError(format!("Failed to convert response: {:?}", e)))?;

    if !response.ok() {
        return Err(PinataError::PinFileError(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    let json = wasm_bindgen_futures::JsFuture::from(response.json()?)
        .await
        .map_err(|e| PinataError::DeserializationError(format!("Failed to parse JSON: {:?}", e)))?;

    json.into_serde()
        .map_err(|e| PinataError::DeserializationError(format!("Failed to deserialize: {:?}", e)))
}

pub async fn pin_json<T: Serialize>(client: &PinataClient, data: &T) -> Result<PinResponse> {
    let json = serde_json::to_string(data)
        .map_err(|e| PinataError::SerializationError(e.to_string()))?;

    let mut headers = Headers::new()
        .map_err(|e| PinataError::NetworkError(format!("Failed to create headers: {:?}", e)))?;
    
    headers.append("Content-Type", "application/json")
        .map_err(|e| PinataError::NetworkError(format!("Failed to set content type: {:?}", e)))?;
    
    headers.append("pinata_api_key", &client.api_key)
        .map_err(|e| PinataError::NetworkError(format!("Failed to set API key: {:?}", e)))?;
    
    headers.append("pinata_secret_api_key", &client.secret_key)
        .map_err(|e| PinataError::NetworkError(format!("Failed to set secret key: {:?}", e)))?;

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.headers(&headers);
    opts.body(Some(&JsValue::from_str(&json)));

    let request = Request::new_with_str_and_init(
        &format!("{}/pinning/pinJSONToIPFS", PINATA_API_URL),
        &opts,
    ).map_err(|e| PinataError::NetworkError(format!("Failed to create request: {:?}", e)))?;

    let window = web_sys::window().unwrap();
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| PinataError::NetworkError(format!("Failed to fetch: {:?}", e)))?;

    let response: Response = response.dyn_into()
        .map_err(|e| PinataError::NetworkError(format!("Failed to convert response: {:?}", e)))?;

    if !response.ok() {
        return Err(PinataError::PinJsonError(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    let json = wasm_bindgen_futures::JsFuture::from(response.json()?)
        .await
        .map_err(|e| PinataError::DeserializationError(format!("Failed to parse JSON: {:?}", e)))?;

    json.into_serde()
        .map_err(|e| PinataError::DeserializationError(format!("Failed to deserialize: {:?}", e)))
}

pub async fn unpin(client: &PinataClient, hash: &str) -> Result<()> {
    let mut headers = Headers::new()
        .map_err(|e| PinataError::NetworkError(format!("Failed to create headers: {:?}", e)))?;
    
    headers.append("pinata_api_key", &client.api_key)
        .map_err(|e| PinataError::NetworkError(format!("Failed to set API key: {:?}", e)))?;
    
    headers.append("pinata_secret_api_key", &client.secret_key)
        .map_err(|e| PinataError::NetworkError(format!("Failed to set secret key: {:?}", e)))?;

    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);
    opts.headers(&headers);

    let request = Request::new_with_str_and_init(
        &format!("{}/pinning/unpin/{}", PINATA_API_URL, hash),
        &opts,
    ).map_err(|e| PinataError::NetworkError(format!("Failed to create request: {:?}", e)))?;

    let window = web_sys::window().unwrap();
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| PinataError::NetworkError(format!("Failed to fetch: {:?}", e)))?;

    let response: Response = response.dyn_into()
        .map_err(|e| PinataError::NetworkError(format!("Failed to convert response: {:?}", e)))?;

    if !response.ok() {
        return Err(PinataError::UnpinError(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    Ok(())
}
