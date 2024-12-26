use std::collections::HashMap;
use lambda_runtime::{Context, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use crate::services::token::details::fetch_token_details;
use crate::services::agent::token_analyze::analyze_token_details;

#[derive(Deserialize, Debug)]
pub struct ApiGatewayPayload {
    #[serde(default)]
    pub queryStringParameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
pub struct ApiGatewayResponse {
    pub statusCode: i32,
    pub body: String,
    pub headers: HashMap<String, String>
}

pub async fn handle(
    event: LambdaEvent<ApiGatewayPayload>,
) -> Result<ApiGatewayResponse, Error> {

    if let Some(method) = event.payload.queryStringParameters.as_ref().and_then(|params| params.get("httpMethod")) {
        if method == "OPTIONS" {
            return Ok(ApiGatewayResponse {
                statusCode: 200,
                headers: cors_headers(),
                body: "".to_string(), // Empty body for preflight response
            });
        }
    }

    // Extract token_id from query parameters
    let token_id = match event.payload.queryStringParameters {
        Some(params) => params.get("token_id").cloned(),
        None => None,
    };

    if token_id.is_none() {
        eprintln!("Missing token_id in query parameters.");
        return Ok(ApiGatewayResponse {
            statusCode: 400,
            headers: cors_headers(),
            body: "Missing token_id query parameter.".to_string(),
        });
    }

    let token_id = token_id.unwrap();
    eprintln!("Fetching details for token_id: {}", token_id);

    // Fetch token details
    match fetch_token_details("solana", &token_id).await {
        Ok(details) => {
            eprintln!("Fetched token detailszzzz: {:?}", details);

            if details.audit.is_none() && details.price.is_none() {
                return Ok(ApiGatewayResponse {
                    statusCode: 404,
                    headers: cors_headers(),
                    body: "Invalid Token - I couldnt find anything related to what was provided".to_string(),
                });
            }

            // Analyze the token details
            let analysis = analyze_token_details(details.audit, details.price).await;

            match analysis {
                Ok(analysis_result) => {
                    Ok(ApiGatewayResponse {
                        statusCode: 200,
                        headers: cors_headers(),
                        body: analysis_result,
                    })
                }
                Err(err) => {
                    eprintln!("Error analyzing token details: {}", err);
                    Ok(ApiGatewayResponse {
                        statusCode: 500,
                        headers: cors_headers(),
                        body: format!("Error analyzing token details: {}", err),
                    })
                }
            }
        }
        Err(err) => {
            eprintln!("Error fetching token details: {}", err);
            Ok(ApiGatewayResponse {
                statusCode: 500,
                headers: cors_headers(),
                body: format!("Failed to fetch token details: {}", err),
            })
        }
    }
}


fn cors_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
    headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, OPTIONS".to_string());
    headers.insert("Access-Control-Allow-Headers".to_string(), "content-type".to_string());
    headers
}

