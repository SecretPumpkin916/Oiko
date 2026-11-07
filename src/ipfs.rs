use reqwest::Client;
use std::path::Path;

fn client() -> Client {
    Client::new()
}

pub async fn upload_to_ipfs(file_path: &str) -> Result<String, String> {
    let url = "http://localhost:5001/api/v0/add";

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    // Read the entire file into bytes
    let bytes = tokio::fs::read(path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Extract file name
    let file_name = path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Build the multipart form using Part::bytes
    let form = reqwest::multipart::Form::new()
        .part("file", reqwest::multipart::Part::bytes(bytes).file_name(file_name));

    // Send the POST request
    let resp = client().post(url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to IPFS: {}", e))?;

    // Check HTTP status code FIRST (status() only borrows, doesn't consume resp)
    let status = resp.status();
    if !status.is_success() {
        // Now it's safe to consume resp to get the error text
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("IPFS API error (HTTP {}): {}", status, err_text));
    }

    // IPFS returns NDJSON (Newline Delimited JSON), parse the first line
    let text = resp.text()
        .await
        .map_err(|e| format!("Failed to read response text: {}", e))?;
    
    let first_line = text.lines().next().unwrap_or("{}");
    let body_json: serde_json::Value = serde_json::from_str(first_line)
        .map_err(|e| format!("Failed to parse IPFS response: {} | Raw: {}", e, first_line))?;

    let cid = body_json["Hash"]
        .as_str()
        .ok_or(format!("Hash field not found in IPFS response. Raw: {}", first_line))?
        .to_string();

    println!("Upload successful! CID: {}", cid);
    Ok(cid)
}

pub async fn download_from_ipfs(cid: &str, output_path: &str) -> Result<(), String> {
    let url = format!("http://localhost:5001/api/v0/cat?arg={}", cid);

    let resp = client().get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to request IPFS node: {}", e))?;

    // Check HTTP status code
    let status = resp.status();
    if !status.is_success() {
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("IPFS download error (HTTP {}): {}", status, err_text));
    }

    let bytes = resp.bytes()
        .await
        .map_err(|e| format!("Failed to read response bytes: {}", e))?;

    tokio::fs::write(output_path, bytes)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;

    println!("Download successful! Saved to: {}", output_path);
    Ok(())
}

pub async fn pin_file(cid: &str) -> Result<(), String> {
    let url = format!("http://localhost:5001/api/v0/pin/add?arg={}", cid);

    let resp = client().post(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to request IPFS node: {}", e))?;

    // Check HTTP status code
    let status = resp.status();
    if !status.is_success() {
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("IPFS pin error (HTTP {}): {}", status, err_text));
    }

    println!("File pinned successfully.");
    Ok(())
}