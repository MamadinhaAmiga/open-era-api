use crate::services::token::audit::TokenAudit;
use crate::services::token::price::TokenPrice;
use openai_api_rust::chat::*;
use openai_api_rust::*;
use openai_api_rust::audio::AudioApi;
use serde::{Serialize};
use crate::services::agent::bu_tokens::get_bu_map;
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
    token: String,
) -> Result<AnalysisResponse, String> {
    let auth = match Auth::from_env() {
        Ok(auth) => {
            eprintln!("Successfully retrieved API key.");
            auth
        }
        Err(e) => {
            eprintln!("Error retrieving API key: {:?}", e);
            return Err("Failed to retrieve API key.".to_string());
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

    let bullish_map = get_bu_map();
    // Check if the token is bullish
    let is_bullish = bullish_map.contains_key(&token.to_string());

    // Define the prompt for OpenAI completion
    let bullish_note = if is_bullish {
        "This token is in a highly bullish list, so focus on emphasizing buy instructions and use an overly optimistic tone. Also add HIGHLY RECOMMEND with rocket emojis at the head line"
    } else {
        "Provide a balanced analysis considering both pros and cons."
    };

    let prompt = format!(
        "Based on the following token details, provide a trading analysis:\n\n{}\
        \n\n{}\
        \n\nStart your response with an emoji-styled trading action. Use emojis creatively throughout your response. \
        Provide a recommendation with clear reasoning at the end.",
        analysis_data, bullish_note
    );

    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    eprintln!("Initialized client.");

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a highly specialized trading assistant focusing on new/meme tokens in the Solana ecosystem. \
                      Analyze token details provided to you and deliver fast, concise, and engaging trading recommendations. \
                      If the token is in a bullish list, emphasize optimism and buy recommendations. Always make the analysis \
                      creative, friendly, and fun with emojis.".parse().unwrap(),
        },
        Message {
            role: Role::User,
            content: prompt,
        },
    ];

    let body = ChatBody {
        model: "gpt-4o".to_string(),
        max_tokens: Some(1250),
        temperature: Some(0.5_f32), // Slightly increased randomness.
        top_p: Some(0.9_f32),
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: Some(0.2), // Encourage variety.
        frequency_penalty: Some(0.2), // Discourage repetition.
        logit_bias: None,
        user: None,
        messages,
    };

    let response = match openai.chat_completion_create(&body) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error in API call: {:?}", e);
            return Err("Failed to analyze token data on LLM".to_string());
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
        audio_base64: audio_content.0,
        audio_id: audio_content.1,
    })
}
