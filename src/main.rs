mod models;
mod config;
mod load_tester;

use std::error::Error;
use tokio::time::Instant;

use config::parse_app_config;
use load_tester::test_load;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Web Load Tester!");
    let config_file_path = "app_config.yaml";

    let endpoints = parse_app_config(config_file_path).await;

    test_load(endpoints).await;


    Ok(())
}
