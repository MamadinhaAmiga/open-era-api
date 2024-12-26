use openai_api_rust::*;
use openai_api_rust::chat::*;
use serde::{Deserialize, Serialize};
use crate::services::token::audit::TokenAudit;
use crate::services::token::price::TokenPrice;

pub async fn analyze_token_details(
    audit: Option<TokenAudit>,
    price: Option<TokenPrice>,
) -> Result<String, String> {
    let auth = match Auth::from_env() {
        Ok(auth) => {
            eprintln!("Successfully retrieved OpenAI API key.");
            auth
        }
        Err(e) => {
            eprintln!("Error retrieving OpenAI API key: {:?}", e);
            return Ok(String::new());
        }
    };

    // Prepare data for the prompt
    let mut analysis_data = String::new();
    if let Some(audit_details) = audit {
        analysis_data.push_str("Audit Details:\n");
        analysis_data.push_str(
            &serde_json::to_string_pretty(&audit_details)
                .map_err(|e| format!("Failed to serialize audit details: {}", e))?,
        );
        analysis_data.push_str("\n");
    }
    if let Some(price_details) = price {
        analysis_data.push_str("Price Details:\n");
        analysis_data.push_str(
            &serde_json::to_string_pretty(&price_details)
                .map_err(|e| format!("Failed to serialize price details: {}", e))?,
        );
    }

    // Define the prompt for OpenAI completion
    let prompt = format!(
        "Analyze the following Solana token details:\n{}\nProvide a short but precise analysis with trade recomendations, you should take into account specially the price variation and from it, recommend a trade action (buy/hold/sell/long/short) its ok to not be sure, just give an action recommendation based on price variation and updatedAt (which means when it was created). if proxy, blacklist and fees are unknown, discard it. ignore contract renounced as well. if its not flagged as scam, say it is not a scam.",
        analysis_data
    );

    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    eprintln!("Initialized OpenAI client.");

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful Solana GPT New/Meme Token Trader assistant. Your goal is to provide a fast and sweet analyze from token details and trade recommendations like buy/hold/short/long/sell.".to_string(),
        },
        Message {
            role: Role::User,
            content: prompt,
        },
    ];

    let body = ChatBody {
        model: "gpt-4o".to_string(),
        max_tokens: Some(1250),
        temperature: Some(0.7_f32),        // Slight randomness.
        top_p: Some(1.0_f32),              // Use all probable tokens.
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages,
    };

    let response = match openai.chat_completion_create(&body) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error in OpenAI API call: {:?}", e);
            return Ok(String::new());
        }
    };

    let message_content = response.choices[0]
        .message
        .as_ref()
        .unwrap()
        .content
        .clone();

    Ok(message_content)
}
