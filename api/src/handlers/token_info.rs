use lambda_runtime::{Context, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use crate::services::token::details::fetch_token_details;

#[derive(Deserialize, Debug)]
pub struct ApiGatewayPayload {
    #[serde(default)]
    pub queryStringParameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
pub struct ApiGatewayResponse {
    pub statusCode: i32,
    pub body: String,
}

pub async fn handle(
    event: LambdaEvent<ApiGatewayPayload>,
) -> Result<ApiGatewayResponse, Error> {
    // Extract token_id from query parameters
    let token_id = match event.payload.queryStringParameters {
        Some(params) => params.get("token_id").cloned(),
        None => None,
    };

    if token_id.is_none() {
        eprintln!("Missing token_id in query parameters.");
        return Ok(ApiGatewayResponse {
            statusCode: 400,
            body: "Missing token_id query parameter.".to_string(),
        });
    }

    let token_id = token_id.unwrap();
    eprintln!("Fetching details for token_id: {}", token_id);

    // Call fetch_token_details from services module
    match fetch_token_details("solana", &token_id).await {
        Ok(details) => {
            eprintln!("Raw token response details: {:?}", details);
            let response_body = serde_json::to_string(&details)
                .map_err(|e| format!("Failed to serialize response: {}", e))?;
            Ok(ApiGatewayResponse {
                statusCode: 200,
                body: response_body,
            })
        }
        Err(err) => {
            eprintln!("Error fetching token details: {}", err);
            Ok(ApiGatewayResponse {
                statusCode: 500,
                body: format!("Failed to fetch token details: {}", err),
            })
        }
    }
}