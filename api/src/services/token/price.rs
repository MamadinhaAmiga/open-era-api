use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenPrice {
    pub price: f64,
    #[serde(default = "default_variation")]
    pub variation5m: f64,
    #[serde(default = "default_variation")]
    pub variation1h: f64,
    #[serde(default = "default_variation")]
    pub variation6h: f64,
    #[serde(default = "default_variation")]
    pub variation24h: f64,
}

fn default_variation() -> f64 {
    0.0
}

pub async fn fetch_token_price(chain: &str, address: &str) -> Result<Option<TokenPrice>, String> {
    // Fetch API key from environment
    let api_key = env::var("DEXTOOLS_API_KEY")
        .map_err(|_| "API key not found in environment variables.".to_string())?;

    let url = format!("https://public-api.dextools.io/standard/v2/token/{}/{}/audit", chain, address);
    let client = Client::new();

    let response = client
        .get(&url)
        .header("X-API-KEY", &api_key)
        .send()
        .await
        .map_err(|err| format!("HTTP request failed: {}", err))?;

    if !response.status().is_success() {
        eprintln!("HTTP request failed: {}", response.status());
        return Ok(None);
    }

    let body = response.text().await.map_err(|err| format!("Failed to read response body: {}", err))?;
    let response_data: serde_json::Value = serde_json::from_str(&body)
        .map_err(|err| format!("Failed to parse response: {}", err))?;

    if let Some(data) = response_data.get("data") {
        let price: TokenPrice = serde_json::from_value(data.clone())
            .map_err(|err| format!("Failed to parse TokenPrice: {}", err))?;
        Ok(Some(price))
    } else {
        Ok(None)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use tokio;

    #[tokio::test]
    async fn test_fetch_token_price_success() {
        dotenv().ok(); // Load environment variables from .env

        // Provide valid chain and token address
        let chain = "solana";
        let address = "mock_token_address";

        let result = fetch_token_price(chain, address).await;

        assert!(result.is_ok(), "Expected successful fetch, got error: {:?}", result);
        let token_price = result.unwrap();
        println!("Token price fetched: {:?}", token_price);
        assert!(token_price.price > 0.0, "Expected price to be greater than 0");
    }

    #[tokio::test]
    async fn test_fetch_token_price_invalid_address() {
        dotenv().ok();

        let chain = "solana";
        let address = "invalid_token_address";

        let result = fetch_token_price(chain, address).await;

        assert!(result.is_err(), "Expected error for invalid address");
        println!("Error received for invalid address: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_fetch_token_price_missing_api_key() {
        dotenv().ok();

        // Temporarily unset the API key environment variable
        std::env::remove_var("DEXTOOLS_API_KEY");

        let chain = "solana";
        let address = "mock_token_address";

        let result = fetch_token_price(chain, address).await;

        assert!(result.is_err(), "Expected error for missing API key");
        assert_eq!(
            result.err().unwrap(),
            "API key not found in environment variables.".to_string()
        );

        // Reset the environment variable
        dotenv().ok();
    }
}
