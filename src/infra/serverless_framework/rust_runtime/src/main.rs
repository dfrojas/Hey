use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(hello_serverless)).await?;
    Ok(())
}

async fn hello_serverless(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(json!({
        "message": "Cool. We deployed to Lambda through Serverless framework."
    }))
}
