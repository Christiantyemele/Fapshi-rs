# Fapshi SDK for Rust
The fapshi-sdk crate provides a type-safe and convenient Rust interface for integrating with the Fapshi payment service API. It enables developers to create payment links, initiate direct payments, query transaction statuses, expire transactions, retrieve transactions by user ID, search transactions, configure webhooks, and check service balance.

## Features

**Authenticated Requests:** Automatically handles apiuser and apikey authentication.
**Sandbox Support:** Test your integration in Fapshi's sandbox environment.
**Modular API:** Separate modules for payments, transactions, webhooks, and balance.
**Error Handling:** Comprehensive error types for HTTP, API, and serialization issues.
**Type Safety:** Uses Rust's strong typing with serde for JSON serialization.

## Installation
Add the following to your Cargo.toml:
```toml
[dependencies]
fapshi-rs = "0.1.0"
```

## Prerequisites

Sign up for a Fapshi account at fapshi.com.
Obtain your apiuser and apikey from the Fapshi dashboard.
For testing, use the sandbox environment; for production, ensure your account is verified.

## Usage
The SDK provides a FapshiClient for making authenticated API requests. Below is an example demonstrating various API operations, as shown in `examples/create_payment.rs`.
### Example: Using the SDK
```rust
use fapshi_rs::{
    api::{balance::BalanceApi, payment::PaymentApi, transaction::TransactionApi},
    client::FapshiClient,
    models::{DirectPaymentRequest, PaymentRequest},
};

/// Example demonstrating how to use the Fapshi SDK for various API operations.
fn main() -> Result<(), Box<dyn std::error::Error>> {
   // Initialize the client in sandbox mode
    dotenv::dotenv().ok();
    let api_user = env::var("FAPSHI_API_USER").expect("FAPSHI_API_USER not set");
    let api_key = env::var("FAPSHI_API_KEY").expect("FAPSHI_API_KEY not set");
    let client = FapshiClient::new(&api_user, &api_key, true)?;

    // Create a payment link
    let payment_request = PaymentRequest {
        amount: 100.0,
        email: Some("yemelechristian2@gmail.com".to_string()),
        redirect_url: Some("https://mywebsite.com".to_string()),
        user_id: Some("abcdef12345".to_string()),
        message: "Pay for play for field way you use to go play for dey for free".to_string(),
        card_only: None,
        external_id: Some("order123".to_string()),
    };

    let payment_response = PaymentApi::create_payment(&client, &payment_request)?;
    println!("\nPayment link: {}\n", payment_response.payment_link);
    println!("\nTransaction ID: {}\n", payment_response.transaction_id);

    // Get transaction Status by transaction ID
    let transaction_status = TransactionApi::get_status(&client, &payment_response.transaction_id)?;
    println!("\nTransaction Status: {:?}\n", transaction_status);

    // Expire a transaction
    TransactionApi::expire_transaction(&client, &payment_response.transaction_id)?;
    println!(
        "\nTransaction {} expired\n",
        payment_response.transaction_id
    );

    // Initiate a direct payment
    let direct_request = DirectPaymentRequest {
        amount: 500.0,
        phone: "670000000".to_string(),
        medium: Some("mobile money".to_string()),
        name: Some("Wilfried".to_string()),
        email: Some("yemelechristian2@gmail.com".to_string()),
        user_id: Some("abcdef12345".to_string()),
        external_id: Some("order123".to_string()),
        message: Some("Direct payment test".to_string()),
    };
    let direct_response = PaymentApi::initiate_direct_payment(&client, &direct_request)?;
    println!(
        "Direct Payment Transaction ID: {}",
        direct_response.transaction_id
    );

    // Get transactions by user ID
    let user_transactions =
        TransactionApi::get_transactions_by_user_id(&client, &transaction_status.user_id.unwrap())?;
    println!("User transactions: {:#?}", user_transactions);

    // Get service balance
    let balance = BalanceApi::get_service_balance(&client)?;
    println!("Service balance: {} {}", balance.balance, balance.currency);

    Ok(())
}
```
**To run the example:**

**Clone the repository:** `git clone https://github.com/Christiantyemele/Fapshi-rs.git`
Replace `"your_api_user"` and `"your_api_key"` in the .env with your Fapshi credentials.
Run the example: 
```
cargo run --example make_payment
```

## Contributing
Contributions are welcome! Please submit issues or pull requests to the GitHub repository.

**Fork the repository.**
Create a feature branch: `git checkout -b feature-name`
Commit your changes: `git commit -m "Add feature"`
Push to the branch: `git push origin feature-name`
Open a pull request.

### License
This project is licensed under the APACHE License. See the LICENSE file for details.

### Support
For questions about the Fapshi API, refer to documentation.fapshi.com or contact Fapshi support. For SDK-specific issues, open a GitHub issue.

Built with 💖 for the FAPSHI service.
