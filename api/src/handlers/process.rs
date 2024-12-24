use lambda_runtime::{Error, LambdaEvent};
use openai_api_rust::*;
use openai_api_rust::chat::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestPayload {
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ResponsePayload {
    pub response: String,
}

pub async fn handle(event: LambdaEvent<RequestPayload>) -> Result<ResponsePayload, Error> {
    let payload = event.payload;

    // Check if the message is empty
    if payload.message.trim().is_empty() {
        return Ok(ResponsePayload {
            response: "The message payload is empty. Please provide a valid input".to_string(),
        });
    }

    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: Role::User,
            content: payload.message,
        },
    ];

    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(50),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages,
    };

    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message_content = choice[0]
        .message
        .as_ref()
        .unwrap()
        .content
        .clone();


    Ok(ResponsePayload { response: message_content })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;
    use crate::handlers::process;

    #[tokio::test]
    async fn test_handle_success() {
        dotenv::dotenv().ok(); // Load environment variables from .env

        // Simulate a Lambda event payload
        let payload = RequestPayload {
            message: "Test message".to_string(),
        };

        // Simulate a Lambda context
        let context = Context::default();

        // Mock OpenAI API response
        let auth = Auth::from_env().unwrap();
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        // Simulate the response
        let result = process::handle(LambdaEvent { payload, context }).await;

        // Assert the result is Ok
        assert!(result.is_ok());

        // Validate the response content
        let response = result.unwrap();
        // Log the response
        println!("Response: {:?}", response);

        assert!(!response.response.is_empty());
    }

    #[tokio::test]
    async fn test_handle_empty_message() {
        dotenv::dotenv().ok(); // Load environment variables from .env

        let payload = RequestPayload {
            message: "".to_string(),
        };

        let context = Context::default();

        let result = process::handle(LambdaEvent { payload, context }).await;

        // Check for a valid response
        assert!(result.is_ok());
        let response = result.unwrap();

        // Log the response
        println!("Response: {:?}", response);

        assert!(response.response.contains("The message payload is empty. Please provide a valid input"));
    }
}

