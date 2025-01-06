# Mavec

[![Latest Version](https://img.shields.io/crates/v/mavec.svg?color=purple)](https://crates.io/crates/mavec/0.1.2)

`mavec` is a lightweight Rust library designed for transforming and handling `Value`s data with ease.
It provides utilities for converting Json-like objects into flattened `Vec<String>` representations, enabling 
seamless integration with applications that require efficient data processing, such as CLI tools, APIs, or data pipelines.

# Getting started

add `mavec` to your dependencies in `Cargo.toml`:

```toml
[dependencies]
mavec = "0.1.0"
```

## Usage

```rust
use mavec::core::to_vec;
use serde_json::json;

fn main() {
    let value = json!({
        "Jeff": true,
        "Rose": "Mary",
        "Miguel": 17,
    });
    
    assert_eq!(
        to_vec(value).unwrap(),
        Vec::from([
            "Jeff".to_string(),
            "true".to_string(),
            "Rose".to_string(),
            "Mary".to_string(),
            "Miguel".to_string(),
            "17".to_string()
        ])
    );
}

```

# Contributing
Contributions are welcome! If you have ideas for new features or optimizations, feel free to open an issue or submit a pull request.

- Fork the repository.
- Create a new branch for your feature or bugfix.
- Submit a pull request with a detailed explanation.
