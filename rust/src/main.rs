use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    msg: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
    tables: Vec<String>,
}

async fn function_handler(ddb_client: &aws_sdk_dynamodb::Client, event: LambdaEvent<Request>) -> Result<Response, Error> {
        // Prepare the response
    let result = ddb_client.list_tables()
        .send().await?;
    let tables = result.table_names.unwrap();
    let resp = Response {
        req_id: event.context.request_id,
        msg: event.payload.msg,
        tables,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let config = aws_config::load_from_env().await;

    // To initialize state that is used by the actual function implementation.
    // https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/basic-shared-resource/src/main.rs#L46
    let ddb_client = &aws_sdk_dynamodb::Client::new(&config);
    run(service_fn(move |event: LambdaEvent<Request>| async move {
        Ok::<Response, Error>(function_handler(ddb_client, event).await.unwrap())
    })).await?;
    Ok(())
}
