use kuzzo::PinataClient;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API credentials from environment variables
    let api_key = env::var("PINATA_API_KEY")?;
    let secret_key = env::var("PINATA_SECRET_KEY")?;

    // Create a new client
    let client = PinataClient::new(api_key, secret_key);

    // Example 1: Pin a file
    println!("Pinning file...");
    let file_result = client.pin_file_blocking("examples/test.txt")?;
    println!("File pinned with hash: {}", file_result.ipfs_hash);

    // Example 2: Pin JSON data
    println!("\nPinning JSON...");
    let json_data = serde_json::json!({
        "name": "test",
        "value": 42
    });
    let json_result = client.pin_json_blocking(&json_data)?;
    println!("JSON pinned with hash: {}", json_result.ipfs_hash);

    // Example 3: Unpin content
    println!("\nUnpinning content...");
    client.unpin_blocking(&file_result.ipfs_hash)?;
    println!("Content unpinned successfully");

    Ok(())
}
