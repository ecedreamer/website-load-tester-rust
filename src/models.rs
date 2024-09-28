

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
    pub response_count: usize,
    pub duration: f64
}

impl TestResult {
    pub fn new(endpoint: Endpoint, request_count: usize, response_count: usize) -> Self {
        Self {
            endpoint,
            request_count,
            response_count,
            duration: 0.0
        }
    }
}