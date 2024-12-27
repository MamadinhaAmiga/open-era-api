use crate::services::token::audit::TokenAudit;
use crate::services::token::price::TokenPrice;
use openai_api_rust::chat::*;
use openai_api_rust::*;
use openai_api_rust::audio::AudioApi;
use serde::{Serialize};
use crate::services::agent::text_to_speech::text_to_speech_with_openai;

#[derive(Serialize)]
pub struct AnalysisResponse {
    pub text: String,
    pub audio_base64: String,
    pub audio_id: String,
}

pub async fn analyze_token_details(
    audit: Option<TokenAudit>,
    price: Option<TokenPrice>,
) -> Result<AnalysisResponse, String> {
    let auth = match Auth::from_env() {
        Ok(auth) => {
            eprintln!("Successfully retrieved OpenAI API key.");
            auth
        }
        Err(e) => {
            eprintln!("Error retrieving OpenAI API key: {:?}", e);
            return Err("Failed to retrieve OpenAI API key.".to_string());
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
        "Knowing that you inherited this data {} ignore Proxy Status, Blacklist, Sell/Buy Taxes and Contract Renounced.\n Please analyse it and Provide a short analysis with telling if its a good trade or not. Use emojis to make it look nice please. Provide a recommendation in the end. Also start the response with a trading action but emoji styled",
        analysis_data
    );

    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    eprintln!("Initialized OpenAI client.");

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful Solana GPT New/Meme Token Trader assistant that. Every data you receive, you should pretend that you already know, so you talk about the data like you are providing it. Your goal is to provide a fast and sweet analyze from token details and trade recommendations like buy/hold/short/long/sell.".to_string(),
        },
        Message {
            role: Role::User,
            content: prompt,
        },
    ];

    let body = ChatBody {
        model: "gpt-4o".to_string(),
        max_tokens: Some(1250),
        temperature: Some(0.3_f32), // Slight randomness.
        top_p: Some(0.9_f32),       // Use all probable tokens.
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
            return Err("Failed to analize token data on LLM".to_string());
        }
    };

    let message_content = response.choices[0]
        .message
        .as_ref()
        .unwrap()
        .content
        .clone();

    let audio_content = text_to_speech_with_openai(&message_content.clone()).await?;

    Ok(AnalysisResponse {
        text: message_content,
        // todo: temp returning empty audios
        audio_base64: audio_content.0,
        audio_id: audio_content.1,
    })
    // Ok(message_content)
}
