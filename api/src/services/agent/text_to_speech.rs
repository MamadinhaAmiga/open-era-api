use reqwest::Client;
use serde::Serialize;
use std::env;
use base64;

#[derive(Serialize)]
struct TtsRequest {
    model: String,
    input: String,
    voice: String,
}

pub async fn text_to_speech_with_openai(text: &str) -> Result<(String, String), String> {
    let api_url = "https://api.openai.com/v1/audio/speech";

    // Get the OpenAI API key from the environment
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set".to_string())?;

    // Create the TTS request payload
    let request_body = TtsRequest {
        model: "tts-1".to_string(),
        input: text.to_string(),
        voice: "nova".to_string(),
    };

    // Send the request to OpenAI TTS API
    let client = Client::new();
    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to OpenAI TTS API: {:?}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "OpenAI TTS API returned an error: {}",
            response.status()
        ));
    }

    // Get the raw audio bytes
    let audio_bytes = response.bytes().await.map_err(|e| {
        format!(
            "Failed to read audio data from OpenAI Text To Speech Model API response: {:?}",
            e
        )
    })?;

    // Encode the audio bytes to Base64
    let audio_base64 = base64::encode(audio_bytes);

    // Return the Base64-encoded audio and a dummy audio ID
    Ok((audio_base64, "openai_audio_id".to_string()))
}
