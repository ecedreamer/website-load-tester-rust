

#[derive(Debug)]
pub struct Endpoint {
    pub path: String,
    pub method: String,
    pub iteration: usize
}



#[derive(Debug)]
pub struct TestResult {
    pub endpoint: Endpoint,
    pub request_count: usize,
    pub success_response_count: usize,
    pub failed_response_count: usize,
    pub throughput: i32,
    pub duration: f64
}

impl TestResult {
    pub fn new(endpoint: Endpoint, request_count: usize) -> Self {
        Self {
            endpoint,
            request_count,
            success_response_count: 0,
            failed_response_count: 0,
            throughput: 0,
            duration: 0.0
        }
    }
}