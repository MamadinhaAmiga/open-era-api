use lambda_runtime::{Error, LambdaEvent};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

pub async fn handle(_: LambdaEvent<()>) -> Result<HealthResponse, Error> {
    Ok(HealthResponse {
        status: "OK".to_string(),
    })
}
