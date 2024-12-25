use lambda_runtime::{service_fn, Error};

// mod handlers;
use open_era_api::handlers::{health, process, status, token_info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok(); // Load .env file once
    println!("Environment loaded!");

    let handler_type = std::env::var("HANDLER_TYPE").unwrap_or_else(|_| "health".to_string());

    match handler_type.as_str() {
        "process" => {
            let func = service_fn(process::handle);
            lambda_runtime::run(func).await?;
        }
        "tokeninfo" => {
            let func = service_fn(token_info::handle);
            lambda_runtime::run(func).await?;
        }
        "status" => {
            let func = service_fn(status::handle);
            lambda_runtime::run(func).await?;
        }
        _ => {
            let func = service_fn(health::handle);
            lambda_runtime::run(func).await?;
        }
    }

    Ok(())
}
