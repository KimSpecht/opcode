use serde::{Deserialize, Serialize};
use tauri::command;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct LmStudioModel {
    id: String,
    object: String,
    owned_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LmStudioModelsResponse {
    object: String,
    data: Vec<LmStudioModel>,
}

/// Fetches available models from LM Studio's /v1/models endpoint
#[command]
pub async fn fetch_lm_studio_models(base_url: String) -> Result<Vec<String>, String> {
    let url = format!("{}/v1/models", base_url.trim_end_matches('/'));
    log::info!("Fetching models from LM Studio at: {}", url);
    
    // Create a client with a timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| {
            log::error!("Failed to create HTTP client: {}", e);
            format!("Failed to create HTTP client: {}", e)
        })?;
    
    // Make the request
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            log::error!("Failed to connect to LM Studio at {}: {}", url, e);
            format!("Failed to connect to LM Studio at {}: {}", url, e)
        })?;
    
    if !response.status().is_success() {
        let error_msg = format!(
            "LM Studio returned status {}: {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown error")
        );
        log::error!("{}", error_msg);
        return Err(error_msg);
    }
    
    // Get response text first for debugging
    let response_text = response
        .text()
        .await
        .map_err(|e| {
            log::error!("Failed to read response text: {}", e);
            format!("Failed to read response text: {}", e)
        })?;
    
    log::debug!("LM Studio response: {}", response_text);
    
    // Parse the response
    let models_response: LmStudioModelsResponse = serde_json::from_str(&response_text)
        .map_err(|e| {
            log::error!("Failed to parse models response: {}. Response was: {}", e, response_text);
            format!("Failed to parse models response: {}. Response was: {}", e, response_text)
        })?;
    
    log::info!("Successfully parsed {} models", models_response.data.len());
    
    // Extract model IDs
    let model_names: Vec<String> = models_response
        .data
        .into_iter()
        .map(|model| {
            log::debug!("Found model: {}", model.id);
            model.id
        })
        .filter(|id| !id.is_empty())
        .collect();
    
    if model_names.is_empty() {
        let error_msg = "No models found in LM Studio. Make sure a model is loaded.".to_string();
        log::warn!("{}", error_msg);
        return Err(error_msg);
    }
    
    log::info!("Returning {} model names: {:?}", model_names.len(), model_names);
    Ok(model_names)
}

/// Tests connection to LM Studio
#[command]
pub async fn test_lm_studio_connection(base_url: String) -> Result<bool, String> {
    let url = format!("{}/v1/models", base_url.trim_end_matches('/'));
    log::info!("Testing connection to LM Studio at: {}", url);
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| {
            log::error!("Failed to create HTTP client for connection test: {}", e);
            format!("Failed to create HTTP client: {}", e)
        })?;
    
    match client.get(&url).send().await {
        Ok(response) => {
            let is_success = response.status().is_success();
            log::info!("Connection test result: {} (status: {})", is_success, response.status());
            Ok(is_success)
        },
        Err(e) => {
            log::warn!("Connection test failed: {}", e);
            Ok(false)
        },
    }
}