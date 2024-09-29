use tokio::time::Instant;
use crate::models::{Endpoint, TestResult};


pub async fn test_endpoint_load(end_point: Endpoint) -> Result<TestResult, String> {
    let client = reqwest::Client::new();
    let iteration = end_point.iteration;
    let mut test_result = TestResult::new(end_point, iteration);
    let mut handles = Vec::with_capacity(iteration);
    for _ in 0..iteration {
        let client = client.clone();
        let path = test_result.endpoint.path.clone();
        let handle = tokio::spawn(async move {
            let result = client.get(path).send().await;
            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        Some(1)
                    } else {
                        None
                    }
                },
                Err(_) => None
            }
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;
    for res in results {
        if let Ok(Some(success)) = res {
            test_result.success_response_count += success;
        } else {
            test_result.failed_response_count += 1;
        }
    }

    Ok(test_result)
}


pub async fn test_load(endpoints: Vec<Endpoint>) {
    for endpoint in endpoints {
        let start_time = Instant::now();
        let result = test_endpoint_load(endpoint).await;
        let elapsed = start_time.elapsed();
        match result {
            Ok(mut test_result) => {
                test_result.duration = elapsed.as_secs_f64();
                println!("{:?}", test_result)
            },
            Err(error) => println!("Error: {}", error),
        }
    }
}