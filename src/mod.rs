mod wasm;

pub use wasm::{pin_file, pin_json, unpin};

use super::{PinataClient, PinResponse, Result, PinataError};
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

const PINATA_API_URL: &str = "https://api.pinata.cloud";

pub async fn pin_file(client: &PinataClient, file_path: &str) -> Result<PinResponse> {
    let result = super::pin_file(client, file_path).await?;
    Ok(result)
}

pub async fn pin_json<T: Serialize>(client: &PinataClient, data: &T) -> Result<PinResponse> {
    let json = serde_json::to_string(data)?;
    let js_value = JsValue::from_str(&json);
    let result = super::pin_json(client, js_value).await?;
    Ok(result)
}

pub async fn unpin(client: &PinataClient, hash: &str) -> Result<()> {
    super::unpin(client, hash).await
}
