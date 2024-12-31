use crate::services::agent::token_analyze::analyze_token_details;
use crate::services::token::details::fetch_token_details;
use lambda_runtime::{Context, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::services::agent::token_slug::token_slug_mapping;

#[derive(Deserialize, Debug)]
pub struct ApiGatewayPayload {
    #[serde(default)]
    pub queryStringParameters: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
pub struct ApiGatewayResponse {
    pub statusCode: i32,
    pub body: String,
    pub headers: HashMap<String, String>,
}

pub async fn handle(event: LambdaEvent<ApiGatewayPayload>) -> Result<ApiGatewayResponse, Error> {
    if let Some(method) = event
        .payload
        .queryStringParameters
        .as_ref()
        .and_then(|params| params.get("httpMethod"))
    {
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

    let mut token_id = token_id.unwrap();
    eprintln!("Fetching details for token_id: {}", token_id);

    if token_id.starts_with("Featured-Token ") {
        if let Some(slug) = token_id.strip_prefix("Featured-Token ") {
            eprintln!("Extracted slug: {}", slug);

            // Map slug to contract address
            if let Some(mapped_token_id) = token_slug_mapping().get(slug) {
                token_id = mapped_token_id.to_string();
                eprintln!("Mapped token_id: {}", token_id);
            } else {
                return Ok(ApiGatewayResponse {
                    statusCode: 404,
                    headers: cors_headers(),
                    body: format!("My system is overloaded right now and I couldnt analyze featured token {} - sorry please retry in a moment", slug),
                });
            }
        }
    }

    // Fetch token details
    match fetch_token_details("solana", &token_id).await {
        Ok(details) => {
            eprintln!("Fetched token detailszzzz: {:?}", details);

            if details.audit.is_none() && details.price.is_none() {
                return Ok(ApiGatewayResponse {
                    statusCode: 404,
                    headers: cors_headers(),
                    body: "Invalid Token - I couldnt find anything related to what was provided"
                        .to_string(),
                });
            }

            // Analyze the token details
            match analyze_token_details(details.audit, details.price, token_id).await {
                Ok(analysis_result) => {
                    // todo: add separate audio transcript
                    // Serialize the `AnalysisResponse` to JSON
                    let response_body = serde_json::to_string(&analysis_result)
                        .map_err(|e| format!("Failed to serialize AnalysisResponse: {}", e))?;

                    Ok(ApiGatewayResponse {
                        statusCode: 200,
                        headers: cors_headers(),
                        body: response_body,
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
    headers.insert(
        "Access-Control-Allow-Methods".to_string(),
        "GET, POST, OPTIONS".to_string(),
    );
    headers.insert(
        "Access-Control-Allow-Headers".to_string(),
        "content-type".to_string(),
    );
    headers
}
