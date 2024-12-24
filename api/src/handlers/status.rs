use lambda_runtime::{Error, LambdaEvent};
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    pub message: String,
}

pub async fn handle(_: LambdaEvent<()>) -> Result<StatusResponse, Error> {
    Ok(StatusResponse {
        message: "Service is up and running.".to_string(),
    })
}
