![Rust](https://img.shields.io/badge/language-rust-orange)
![Coverage](https://img.shields.io/badge/coverage-84.36%25-green)
![License](https://img.shields.io/badge/license-MIT%20%7C%20GPL-blue)

# HTTP Email API → SMTP

A lightweight HTTP API that sends emails through an SMTP server.

This project was created while working through **Chapter 7 of _Zero to Production in Rust_**.  
Instead of using Postmark, this implementation allows you to send emails through **your own SMTP server**.

This can be useful if:

- You encounter network issues connecting to Postmark
- You prefer not to share credentials or information with a third-party email provider
- You want to experiment with your own email infrastructure

The project will continue to evolve as I progress through the book and improve the implementation.

---

# API Example

Send an email using the HTTP API:

```bash
curl -i "http://localhost:8080/email" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: <password>" \
  -d '{
    "From": "a@send.com",
    "To": "b@receive.com",
    "Subject": "From http2smtp",
    "TextBody": "The program succeeded.",
    "HtmlBody": "<html><body><strong>The program</strong> succeeded.</body></html>"
  }'
```

---

# Running the Server

```bash
cargo run
```

The server will start on:

```
http://localhost:8080
```

Configuration is loaded from the `configuration/` directory.

Two template files are provided:
```
configuration/base-temp.yaml
configuration/local-temp.yaml
```
Before running the application:

Copy base-temp.yaml to base.yaml and fill in the required configuration values.

Copy local-temp.yaml to local.yaml (no modification is required).

Example:
```
cp configuration/base-temp.yaml configuration/base.yaml
cp configuration/local-temp.yaml configuration/local.yaml
```
---

# Testing

Tests are written using Rust’s built-in test framework and async tests via `tokio`.

Test coverage is measured with:

```
cargo llvm-cov
```

Current coverage:

| Metric    | Coverage   |
|-----------|------------|
| Lines     | **84.36%** |
| Regions   | **83.59%** |
| Functions | **81.82%** |

Generate an HTML coverage report:

```bash
cargo llvm-cov --html
```

The report will be available at:

```
target/llvm-cov/html/index.html
```

---

# Project Structure

```
src
├── configuration.rs
├── email
│   ├── api_models.rs
│   ├── domain.rs
│   ├── transport.rs
│   └── response.rs
├── send_email
│   ├── message_id.rs
│   ├── service.rs
│   └── smtp.rs
├── service
│   ├── auth.rs
│   ├── email_front.rs
│   ├── smtp_provider.rs
│   └── errors.rs
├── telemetry
│   ├── init.rs
│   └── span.rs
├── setup.rs
└── main.rs
```

---

# Useful Notes

## Multiple Git Remotes

```bash
git remote set-url --add origin https://<repo>
```

## Inspect HTTP Request Headers

```
let key_list = req_headers.keys();
for key in key_list {
    let name = key.as_str();
    println!(
        "{}: {}",
        name,
        req_headers.get(name).unwrap().to_str().unwrap()
    );
}
```

---

# License

Dual-licensed under either:

- MIT License
- GPL License