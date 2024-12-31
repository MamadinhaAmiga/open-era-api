use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct SellBuyTax {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAudit {
    pub isOpenSource: String,
    pub isHoneypot: String,
    pub isMintable: String,
    pub isProxy: String,
    pub slippageModifiable: String,
    pub isBlacklisted: String,
    pub sellTax: SellBuyTax,
    pub buyTax: SellBuyTax,
    pub isContractRenounced: String,
    pub isPotentiallyScam: String,
    pub updatedAt: String,
}

pub async fn fetch_token_audit(chain: &str, address: &str) -> Result<Option<TokenAudit>, String> {
    // Fetch API key from environment
    let api_key = env::var("DEXTOOLS_API_KEY")
        .map_err(|_| "API key not found in environment variables.".to_string())?;

    let url = format!("https://public-api.dextools.io/standard/v2/token/{}/{}/audit", chain, address);
    let client = Client::new();

    // Send the HTTP request
    let response = client
        .get(&url)
        .header("X-API-KEY", &api_key)
        .send()
        .await
        .map_err(|err| format!("HTTP request failed: {}", err))?;

    let status = response.status();
    if !status.is_success() {
        eprintln!(
            "Request failed with status: {}. Returning None.",
            status
        );
        return Ok(None); // Return None for non-successful status
    }
    // Clone response body for logging

    // Parse the response directly into TokenAudit if available
    let parsed_response: serde_json::Value = response
        .json()
        .await
        .map_err(|err| format!("Failed to parse JSON response: {}", err))?;

    // Check if "data" exists and deserialize it
    if let Some(data) = parsed_response.get("data") {
        let audit: TokenAudit = serde_json::from_value(data.clone())
            .map_err(|err| format!("Failed to parse audit data: {}", err))?;
        Ok(Some(audit)) // Return the TokenAudit inside Some
    } else {
        eprintln!("No 'data' field found in the response. Returning None.");
        Ok(None) // Return None if "data" is missing
    }
}
