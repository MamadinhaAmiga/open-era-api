use super::audit::fetch_token_audit;
use super::price::fetch_token_price;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TokenDetails {
    pub audit: Option<super::audit::TokenAudit>,
    pub price: Option<super::price::TokenPrice>,
}

pub async fn fetch_token_details(chain: &str, address: &str) -> Result<TokenDetails, String> {
    let audit = match fetch_token_audit(chain, address).await {
        Ok(Some(audit)) => Some(audit), // Unwrap the inner Option
        Ok(None) => None,              // Explicitly handle None
        Err(err) => {
            eprintln!("Failed to fetch token audit: {}", err);
            None
        }
    };
    let price = match fetch_token_price(chain, address).await {
        Ok(Some(data)) => Some(data),
        Ok(None) => None,              // Explicitly handle None
        Err(err) => {
            eprintln!("Error in fetch_token_price: {}", err);
            None
        },
    };


    eprintln!("Handler called for token info.");

    Ok(TokenDetails {
        audit,
        price,
    })
}
