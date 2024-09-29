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

