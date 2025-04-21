use super::super::{PinResponse, PinataClient, PinataError, Result};
use serde::Serialize;
use std::fs;

const PINATA_API_URL: &str = "https://api.pinata.cloud";

pub fn pin_file(client: &PinataClient, file_path: &str) -> Result<PinResponse> {
    let file_content = fs::read(file_path).map_err(|e| PinataError::PinFileError(e.to_string()))?;

    let form = reqwest::blocking::multipart::Form::new().part(
        "file",
        reqwest::blocking::multipart::Part::bytes(file_content).file_name(file_path.to_string()),
    );

    let response = reqwest::blocking::Client::new()
        .post(format!("{}/pinning/pinFileToIPFS", PINATA_API_URL))
        .header("pinata_api_key", &client.api_key)
        .header("pinata_secret_api_key", &client.secret_key)
        .multipart(form)
        .send()
        .map_err(|e| PinataError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(PinataError::PinFileError(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    response
        .json()
        .map_err(|e| PinataError::DeserializationError(e.to_string()))
}

pub fn pin_json<T: Serialize>(client: &PinataClient, data: &T) -> Result<PinResponse> {
    let response = reqwest::blocking::Client::new()
        .post(format!("{}/pinning/pinJSONToIPFS", PINATA_API_URL))
        .header("pinata_api_key", &client.api_key)
        .header("pinata_secret_api_key", &client.secret_key)
        .json(data)
        .send()
        .map_err(|e| PinataError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(PinataError::PinJsonError(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    response
        .json()
        .map_err(|e| PinataError::DeserializationError(e.to_string()))
}

pub fn unpin(client: &PinataClient, hash: &str) -> Result<()> {
    let response = reqwest::blocking::Client::new()
        .delete(format!("{}/pinning/unpin/{}", PINATA_API_URL, hash))
        .header("pinata_api_key", &client.api_key)
        .header("pinata_secret_api_key", &client.secret_key)
        .send()
        .map_err(|e| PinataError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(PinataError::UnpinError(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    Ok(())
}
