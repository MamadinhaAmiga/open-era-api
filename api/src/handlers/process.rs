use lambda_runtime::{Error, LambdaEvent};
use openai_api_rust::*;
use openai_api_rust::chat::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestPayload {
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ResponsePayload {
    pub response: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiGatewayPayload {
    pub body: String,
}

#[derive(Serialize, Debug, Default)]
pub struct ApiGatewayResponse {
    pub statusCode: i32,
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
}


pub async fn handle(
    event: LambdaEvent<ApiGatewayPayload>,
) -> Result<ApiGatewayResponse, Error> {
    eprintln!("Raw API Gateway event: {:?}", event);

    let payload: RequestPayload = serde_json::from_str(&event.payload.body)
        .map_err(|e| format!("Failed to parse body: {}", e))?;

    eprintln!("Parsed body: {:?}", payload);

    if payload.message.trim().is_empty() {
        eprintln!("Error: Empty payload received.");
        return Ok(ApiGatewayResponse {
            statusCode: 400,
            body: Some("Message payload is empty.".to_string()),
            ..Default::default()
        });
    }

    eprintln!("Payload message: {}", payload.message);

    let auth = match Auth::from_env() {
        Ok(auth) => {
            eprintln!("Successfully retrieved OpenAI API key.");
            auth
        }
        Err(e) => {
            eprintln!("Error retrieving OpenAI API key: {:?}", e);
            return Ok(ApiGatewayResponse {
                statusCode: 500,
                body: Some("Failed to retrieve OpenAI API key.".to_string()),
                ..Default::default()
            });
        }
    };

    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    eprintln!("Initialized OpenAI client.");

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: Role::User,
            content: payload.message.clone(),
        },
    ];

    eprintln!("Constructed OpenAI chat messages: {:?}", messages);

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

    eprintln!("Constructed OpenAI request body: {:?}", body);

    let response = match openai.chat_completion_create(&body) {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error in OpenAI API call: {:?}", e);
            return Ok(ApiGatewayResponse {
                statusCode: 500,
                body: Some("Failed to call OpenAI API.".to_string()),
                ..Default::default()
            });
        }
    };

    eprintln!("Received OpenAI API response: {:?}", response);

    let message_content = response.choices[0]
        .message
        .as_ref()
        .unwrap()
        .content
        .clone();

    eprintln!("Parsed response message: {}", message_content);

    Ok(ApiGatewayResponse {
        statusCode: 200,
        body: Some(message_content),
        ..Default::default()
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};
    use crate::handlers::process;

    #[tokio::test]
    async fn test_handle_success() {
        dotenv::dotenv().ok(); // Load environment variables from .env

        // Simulate a Lambda event payload
        let payload = ApiGatewayPayload {
            body: serde_json::to_string(&RequestPayload {
                message: "Test message".to_string(),
            })
                .unwrap(),
        };

        // Simulate a Lambda context
        let context = Context::default();

        // Simulate the Lambda event
        let event = LambdaEvent {
            payload,
            context,
        };

        // Call the handler
        let result = process::handle(event).await;

        // Assert the result is Ok
        assert!(result.is_ok());

        // Validate the response content
        let response = result.unwrap();
        // Log the response
        println!("Response: {:?}", response);

        // Check the body of the response
        assert!(response.body.is_some());
        assert!(!response.body.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_handle_empty_message() {
        dotenv::dotenv().ok(); // Load environment variables from .env

        // Simulate a Lambda event payload with an empty message
        let payload = ApiGatewayPayload {
            body: serde_json::to_string(&RequestPayload {
                message: "".to_string(),
            })
                .unwrap(),
        };

        let context = Context::default();

        // Simulate the Lambda event
        let event = LambdaEvent {
            payload,
            context,
        };

        // Call the handler
        let result = process::handle(event).await;

        // Assert the result is Ok
        assert!(result.is_ok());

        // Validate the response content
        let response = result.unwrap();
        // Log the response
        println!("Response: {:?}", response);

        // Check the body of the response
        assert!(response.body.is_some());
        assert!(response
            .body
            .unwrap()
            .contains("Message payload is empty."));
    }
}


