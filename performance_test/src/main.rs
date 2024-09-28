use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use aws_sdk_lambda::primitives::Blob;
use futures::future::join_all;
use thiserror::Error;
use uuid::Uuid;
use crate::PerformanceTestError::{ExecutionFailed, ForceColdStartFailure, InvokeFailed};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

const FUNCTION_NAMES: [&str; 3] = [
    "JavaLambda",
    "JavaSnapStartLambda",
    "RustLambda"
];

const PAYLOAD: &str = "{\"msg\": \"Oh, hey there!\"}";

/// The number of requests to make for each stage of testing.
const REQUEST_COUNT: usize = 3;

#[derive(Debug, Error)]
enum PerformanceTestError {
    #[error("Unable to invoke the function")]
    InvokeFailed,
    #[error("Execution of the function failed.")]
    ExecutionFailed,
    #[error("An error occurred while trying to force a cold start.")]
    ForceColdStartFailure,
}

#[derive(Debug)]
struct PerformanceTestResults<'a> {
    function_name: &'a str,
    standard_invokes: Vec<u128>,
    cold_start_invokes: Vec<u128>,
}

struct PerformanceTest<'a> {
    function_name: &'a str,
    lambda_client: Arc<aws_sdk_lambda::Client>,
}

impl<'a> PerformanceTest<'a> {
    fn new(function_name: &'a str, lambda_client: &Arc<aws_sdk_lambda::Client>) -> Self {
        Self {
            function_name,
            lambda_client: lambda_client.clone(),
        }
    }

    pub async fn test(&self) -> Result<PerformanceTestResults, PerformanceTestError> {

        // Cold start testing.
        // TODO: These need to be run synchronously.
        let mut cold_start_invokes: Vec<u128> = vec![];
        for n in 1..=REQUEST_COUNT {
            cold_start_invokes.push(self.cold_start_invoke().await?);
        }

        // Standard invoke testing.
        // This doesn't attempt to do any warming of the function.
        let mut standard_invokes: Vec<u128> = vec![];
        for n in 1..= REQUEST_COUNT {
            standard_invokes.push(self.invoke().await?);
        }

        Ok(PerformanceTestResults {
            function_name: self.function_name,
            standard_invokes,
            cold_start_invokes,
        })
    }

    async fn cold_start_invoke(&self) -> Result<u128, PerformanceTestError> {
        self.force_cold_start().await?;
        self.invoke().await
    }

    /// Makes a simple change to the active Lambda to provoke a cold start.
    /// As described here: https://stackoverflow.com/a/78062340/1060627
    async fn force_cold_start(&self) -> Result<(), PerformanceTestError> {
        let new_description = Uuid::new_v4().to_string();
        match self.lambda_client.update_function_configuration()
            .function_name(self.function_name)
            .description(new_description)
            .send().await {
            Ok(_) => {
                thread::sleep(Duration::from_secs(1));
                Ok(())
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                Err(ForceColdStartFailure)
            },
        }
    }

    async fn invoke(&self) -> Result<u128, PerformanceTestError> {
        let start = Self::now_in_millis();
        match self.lambda_client.invoke()
            .function_name(self.function_name)
            .payload(Blob::new(PAYLOAD))
            .send().await {
            Ok(result) => {
                if let Some(error) = result.function_error {
                    eprintln!("An error occurred: {}", error);
                    Err(ExecutionFailed)
                } else {
                    let end = Self::now_in_millis();
                    let execution_time = end - start;
                    eprintln!("Execution of {} function took {} msec.", self.function_name, execution_time);
                    Ok(execution_time)
                }
            }
            Err(_) => Err(InvokeFailed)
        }
    }

    fn now_in_millis() -> u128 {
        let start = SystemTime::now();
        start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_millis()
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), PerformanceTestError> {
    let config = aws_config::load_from_env().await;
    let lambda_client = Arc::new(aws_sdk_lambda::Client::new(&config));
    let testers =
        FUNCTION_NAMES.iter()
            .map(|function_name| PerformanceTest::new(function_name, &lambda_client))
            .collect::<Vec<_>>();
    let testings =
        testers.iter().map(|tester| tester.test())
            .collect::<Vec<_>>();
    let results: Vec<PerformanceTestResults> =
        join_all(testings).await.into_iter().collect::<Result<Vec<_>, PerformanceTestError>>()?;
    eprintln!("All results: {:?}", results);
    Ok(())
}
