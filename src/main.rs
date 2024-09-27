use std::error::Error;
use tokio::fs;
use tokio::time::Instant;
use yaml_rust2::YamlLoader;


#[derive(Debug)]
struct Endpoint {
    path: String,
    method: String,
}



#[derive(Debug)]
struct TestResult {
    endpoint: Endpoint,
    request_count: usize,
    response_count: usize,
}

impl TestResult {
    fn new(endpoint: Endpoint, request_count: usize, response_count: usize) -> Self {
        Self {
            endpoint,
            request_count,
            response_count
        }
    }
}


async fn test_load(end_point: Endpoint, iteration: usize) -> Result<TestResult, String> {
    let client = reqwest::Client::new();
    let mut test_result = TestResult::new(end_point, iteration, 0);
    let mut handles = Vec::with_capacity(iteration);
    for _ in 0..iteration {
        let client = client.clone();
        let path = test_result.endpoint.path.clone();
        let handle = tokio::spawn(async move {
            let result = client.get(path).send().await;
            match result {
                Ok(response) => Some(1),
                Err(_) => None
            }
        });
        handles.push(handle);

        // let result = client.get(&test_result.endpoint.path).send().await;
        // match result {
        //     Ok(response) => {
        //         if response.status().is_success() {
        //             test_result.response_count += 1;
        //         }
        //     }
        //     Err(_) => {}
        // }

    }

    let results = futures::future::join_all(handles).await;
    for res in results {
        if let Ok(Some(success)) = res {
            test_result.response_count += success;
        }
    }

    Ok(test_result)
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Web Load Tester!");
    let contents = fs::read_to_string("app_config.yaml").await?;
    let yaml_contents = YamlLoader::load_from_str(&contents)?;
    let base_url = yaml_contents[0]["base_url"].as_str().unwrap();
    let endpoints_yaml = yaml_contents[0]["endpoints"].as_vec().unwrap();


    let endpoints: Vec<Endpoint> = endpoints_yaml
        .iter()
        .map(|entry| {
            let path = format!("{}{}", base_url, entry["path"].as_str().unwrap());
            Endpoint {
                path,
                method: entry["method"].as_str().unwrap().to_string(),
            }
        })
        .collect();

    for endpoint in endpoints {
        let start_time = Instant::now();
        let result = test_load(endpoint, 1000).await;
        let elapsed = start_time.elapsed();
        match result {
            Ok(test_result) => println!("{:?} | {:?}", test_result, elapsed),
            Err(error) => println!("Error: {}", error),
        }
    }

    Ok(())
}
