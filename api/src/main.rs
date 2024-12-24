use lambda_runtime::{service_fn, Error};

mod handlers;
use handlers::{health, process, status};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let handler_type = std::env::var("HANDLER_TYPE").unwrap_or_else(|_| "health".to_string());

    match handler_type.as_str() {
        "process" => {
            let func = service_fn(process::handle);
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
