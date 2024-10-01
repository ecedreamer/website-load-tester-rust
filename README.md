## Web Application Load Tester in Rust Lang
This is a simple web application load tester for testing the load handling capacity of defined endpoints. It is written in Rust Programming Language.

### Tech Stacks Used
1. Rust Programming Language
2. Tokio for Async Runtime
3. Reqwest as web client

#### Cargo.toml
```toml
tokio = { version = "1.39.3", features = ["full"] }
yaml-rust2 = "0.9.0"
reqwest = "0.12.7"
futures = "0.3.30"
```

### Usase
1. Clone the project
2. Define your endpoints in the app_config.yml
3. Run the project in release mode.
    ```bash
    $ cargo run --release
    ```
   
### Sample Output
```shell
Starting Web Load Tester!
TestResult { endpoint: Endpoint { path: "http://127.0.0.1:8080/", method: "GET", iteration: 1000 }, request_count: 1000, success_response_count: 149, failed_response_count: 851, throughput: 2621, duration: 0.056841375 }
TestResult { endpoint: Endpoint { path: "http://127.0.0.1:8080/contact", method: "GET", iteration: 1500 }, request_count: 1500, success_response_count: 1169, failed_response_count: 331, throughput: 6909, duration: 0.16919925 }

```

