// use openai_api_rust::*;
// use serde::Serialize;
// use crate::services::token::audit::TokenAudit;
// use crate::services::token::price::TokenPrice;
//
// #[derive(Serialize)]
// pub struct AnalysisResponse {
//     pub text: String,
//     pub audio_base64: String,
//     pub audio_id: String,
// }
//
// pub async fn analyze_token_details(
//     audit: Option<TokenAudit>,
//     price: Option<TokenPrice>,
// ) -> Result<AnalysisResponse, String> {
//     let auth = match Auth::from_env() {
//         Ok(auth) => {
//             eprintln!("Successfully retrieved OpenAI API key.");
//             auth
//         }
//         Err(e) => {
//             eprintln!("Error retrieving OpenAI API key: {:?}", e);
//             return Err("Failed to retrieve OpenAI API key.".to_string());
//         }
//     };
//
//     // Prepare data for the prompt
//     let mut analysis_data = String::new();
//     if let Some(audit_details) = audit {
//         analysis_data.push_str("Audit Details:\n");
//         analysis_data.push_str(
//             &serde_json::to_string_pretty(&audit_details)
//                 .map_err(|e| format!("Failed to serialize audit details: {}", e))?,
//         );
//         analysis_data.push_str("\n");
//     }
//     if let Some(price_details) = price {
//         analysis_data.push_str("Price Details:\n");
//         analysis_data.push_str(
//             &serde_json::to_string_pretty(&price_details)
//                 .map_err(|e| format!("Failed to serialize price details: {}", e))?,
//         );
//     }
//     eprintln!("Analysis Data: prepared!");
//
//     // Define the prompt for OpenAI completion
//     let prompt = format!(
//         "Analyze the following Solana token details:\n{}\nProvide a short but precise analysis with trade recomendations, you should take into account specially the price variation and from it, recommend a trade action (buy/hold/sell/long/short) its ok to not be sure, just give an action recommendation based on price variation and updatedAt (which means when it was created). if proxy, blacklist and fees are unknown, discard it. ignore contract renounced as well. if its not flagged as scam, say it is not a scam.",
//         analysis_data
//     );
//
//     let messages = vec![
//         Message {
//             role: Role::System,
//             content: "You are a helpful Solana GPT New/Meme Token Trader assistant. Your goal is to provide a fast and sweet analyze from token details and trade recommendations like buy/hold/short/long/sell.".to_string(),
//         },
//         Message {
//             role: Role::User,
//             content: prompt,
//         },
//     ];
//
//     let body = serde_json::json!({
//         "model": "gpt-4o-audio-preview",
//         "modalities": ["text", "audio"],
//         "audio": { "voice": "nova", "format": "wav" },
//         "messages": messages,
//     });
//
//     eprintln!("Sending request!");
//     let client = reqwest::Client::new();
//     let response = client
//         .post("https://api.openai.com/v1/chat/completions")
//         .header("Authorization", format!("Bearer {}", auth.api_key))
//         .json(&body)
//         .send()
//         .await
//         .map_err(|err| format!("HTTP request failed: {}", err))?;
//
//     if !response.status().is_success() {
//         return Err(format!(
//             "OpenAI API request failed with status: {}",
//             response.status()
//         ));
//     }
//
//     eprintln!("Before repsonse body!");
//
//     let response_body: serde_json::Value = response
//         .json()
//         .await
//         .map_err(|err| format!("Failed to parse OpenAI API response: {}", err))?;
//
//     // eprintln!("OpenAI API raw response choices: {:?}", response_body["choices"]);
//
//     // Safely parse the response to extract audio and transcript
//     let choices = response_body.get("choices")
//         .and_then(|choices| choices.as_array())
//         .ok_or_else(|| "Missing or invalid 'choices' in OpenAI API response.".to_string())?;
//
//     let first_choice = choices.get(0)
//         .ok_or_else(|| "No choices available in OpenAI API response.".to_string())?;
//
//     let message = first_choice.get("message")
//         .ok_or_else(|| "Missing 'message' in choice.".to_string())?;
//
//     // Extract the audio data
//     let audio = message.get("audio")
//         .ok_or_else(|| "Missing 'audio' in message.".to_string())?;
//
//     // Extract the transcript (fallback to empty if not present)
//     let transcript = audio.get("transcript")
//         .and_then(|transcript| transcript.as_str())
//         .unwrap_or("")
//         .to_string();
//
//     let audio_id = audio.get("id")
//         .and_then(|id| id.as_str())
//         .ok_or_else(|| "Missing 'id' in audio.".to_string())?
//         .to_string();
//
//     let audio_base64 = audio["data"]
//         .as_str()
//         .ok_or_else(|| "Missing audio data.".to_string())?
//         .to_string();
//
//     eprintln!("text : {}", transcript);
//     eprintln!("audio_base64 (first 50 chars): {}", &audio_base64.chars().take(50).collect::<String>());
//
//     Ok(AnalysisResponse {
//         text: transcript,
//         audio_base64,
//         audio_id,
//     })
// }
