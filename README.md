# Fapshi SDK for Rust

The `fapshi-rs` crate provides a type-safe and convenient Rust interface for integrating with the Fapshi payment service API. It enables developers to create payment links, initiate direct payments, query transaction statuses, expire transactions, retrieve transactions by user ID, configure webhooks, and check service balance.

## Features

- **Authenticated Requests**: Automatically handles `apiuser` and `apikey` authentication
- **Sandbox Support**: Test your integration in Fapshi's sandbox environment
- **Modular API**: Separate modules for payments, transactions, webhooks, and balance
- **Error Handling**: Comprehensive error types for HTTP, API, and serialization issues
- **Type Safety**: Uses Rust's strong typing with serde for JSON serialization
- **Async Support**: Optional asynchronous API calls with the `async` feature, ideal for async runtimes like Tokio

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fapshi-rs = "0.2.0"
```

For asynchronous support, enable the `async` feature:

```toml
[dependencies]
fapshi-rs = { version = "0.2.0", features = ["async"] }
```

## Prerequisites

1. Sign up for a Fapshi account at [fapshi.com](https://fapshi.com)
2. Obtain your `apiuser` and `apikey` from the Fapshi dashboard
3. For testing, use the sandbox environment; for production, ensure your account is verified

## Usage

The SDK provides a `FapshiClient` for making authenticated API requests. Below are examples demonstrating both synchronous and asynchronous API operations.

### Example: Synchronous Usage

```rust
use fapshi_rs::{
    api::{balance::BalanceApi, payment::PaymentApi, transaction::TransactionApi},
    client::FapshiClient,
    models::{DirectPaymentRequest, PaymentRequest},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client in sandbox mode
    let api_user = std::env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
    let api_key = std::env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
    let client = FapshiClient::new(&api_user, &api_key, true)?;

    // Create a payment link
    let payment_request = PaymentRequest {
        amount: 100.0,
        currency: "USD".to_string(),
        description: "Test payment".to_string(),
        customer_email: Some("yemelechristian2@gmail.com".to_string()),
        user_id: Some("abcdef12345".to_string()),
    };
    let payment_response = PaymentApi::create_payment(&client, &payment_request)?;
    println!("Payment link: {}", payment_response.payment_link);

    // Get transaction status
    let transaction_status = TransactionApi::get_status(&client, &payment_response.transaction_id)?;
    println!("Transaction Status: {:?}", transaction_status);

    // Get service balance
    let balance = BalanceApi::get_service_balance(&client)?;
    println!("Service balance: {} {}", balance.balance, balance.currency);

    Ok(())
}
```

### Example: Asynchronous Usage

```rust
use fapshi_rs::{
    api::{balance::BalanceApi, payment::PaymentApi, transaction::TransactionApi},
    client::FapshiClient,
    models::{DirectPaymentRequest, PaymentRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client in sandbox mode
    let api_user = std::env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
    let api_key = std::env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
    let client = FapshiClient::new(&api_user, &api_key, true)?;

    // Create a payment link
    let payment_request = PaymentRequest {
        amount: 100.0,
        currency: "USD".to_string(),
        description: "Test payment".to_string(),
        customer_email: Some("yemelechristian2@gmail.com".to_string()),
        user_id: Some("abcdef12345".to_string()),
    };
    let payment_response = PaymentApi::create_payment(&client, &payment_request).await?;
    println!("Payment link: {}", payment_response.payment_link);

    // Get transaction status
    let transaction_status = TransactionApi::get_status(&client, &payment_response.transaction_id).await?;
    println!("Transaction Status: {:?}", transaction_status);

    // Get service balance
    let balance = BalanceApi::get_service_balance(&client).await?;
    println!("Service balance: {} {}", balance.balance, balance.currency);

    Ok(())
}
```

## Running the Examples

To run the examples:

1. Clone the repository:
   ```bash
   git clone https://github.com/Christiantyemele/Fapshi-rs.git
   ```

2. Create a `.env` file with your Fapshi credentials:
   ```env
   FAPSHI_API_USER=your_api_user
   FAPSHI_API_KEY=your_api_key
   ```

3. Run a synchronous example:
   ```bash
   cargo run --example sync_payment
   ```

4. Run an asynchronous example (with async feature enabled):
   ```bash
   cargo run --features async --example async_payment
   ```

## Contributing

Contributions are welcome! Please submit issues or pull requests to the GitHub repository.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -m "Add feature"`
4. Push to the branch: `git push origin feature-name`
5. Open a pull request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

For questions about the Fapshi API, refer to [documentation.fapshi.com](https://documentation.fapshi.com) or contact Fapshi support. For SDK-specific issues, open a GitHub issue.

---

Built with 💖 for the Fapshi service.