use tokio::fs;
use yaml_rust2::YamlLoader;
use crate::models::Endpoint;

pub async fn parse_app_config(config_file: &str) -> Vec<Endpoint> {
    let contents = fs::read_to_string(config_file).await.unwrap();
    let yaml_contents = YamlLoader::load_from_str(&contents).unwrap();
    let base_url = yaml_contents[0]["base_url"].as_str().unwrap();
    let endpoints_yaml = yaml_contents[0]["endpoints"].as_vec().unwrap();
    let endpoints: Vec<Endpoint> = endpoints_yaml
        .iter()
        .map(|entry| {
            let path = format!("{}{}", base_url, entry["path"].as_str().unwrap());
            Endpoint {
                path,
                method: entry["method"].as_str().unwrap().to_string(),
                iteration: entry["iteration"].as_i64().unwrap() as usize
            }
        })
        .collect();
    endpoints
}

