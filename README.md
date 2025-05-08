# Fapshi SDK for Rust
The fapshi-sdk crate provides a type-safe and convenient Rust interface for integrating with the Fapshi payment service API. It enables developers to create payment links, initiate direct payments, query transaction statuses, expire transactions, retrieve transactions by user ID, search transactions, configure webhooks, and check service balance.
Features

Authenticated Requests: Automatically handles apiuser and apikey authentication.
Sandbox Support: Test your integration in Fapshi's sandbox environment.
Modular API: Separate modules for payments, transactions, webhooks, and balance.
Error Handling: Comprehensive error types for HTTP, API, and serialization issues.
Type Safety: Uses Rust's strong typing with serde for JSON serialization.

## Installation
Add the following to your Cargo.toml:
```toml
[dependencies]
fapshi-sdk = "0.1.0"
```

## Prerequisites

Sign up for a Fapshi account at fapshi.com.
Obtain your apiuser and apikey from the Fapshi dashboard.
For testing, use the sandbox environment; for production, ensure your account is verified.

## Usage
The SDK provides a FapshiClient for making authenticated API requests. Below is an example demonstrating various API operations, as shown in `examples/create_payment.rs`.
### Example: Using the SDK
```rust
use fapshi_sdk::{
    FapshiClient,
    models::{PaymentRequest, DirectPaymentRequest, TransactionSearchQuery},
    api::{
        payment::PaymentApi,
        transaction::TransactionApi,
        balance::BalanceApi,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client in sandbox mode
    let client = FapshiClient::new("your_api_user", "your_api_key", true)?;

    // Create a payment link
    let payment_request = PaymentRequest {
        amount: 100.0,
        currency: "USD".to_string(),
        description: "Test payment".to_string(),
        customer_email: Some("test@example.com".to_string()),
        user_id: Some("user123".to_string()),
    };
    let payment_response = PaymentApi::create_payment(&client, &payment_request)?;
    println!("Payment link: {}", payment_response.payment_link);

    // Initiate a direct payment
    let direct_request = DirectPaymentRequest {
        amount: 50.0,
        currency: "USD".to_string(),
        phone_number: "1234567890".to_string(),
        description: "Direct payment test".to_string(),
    };
    let direct_response = PaymentApi::initiate_direct_payment(&client, &direct_request)?;
    println!("Direct Payment Transaction ID: {}", direct_response.transaction_id);

    // Expire a transaction
    TransactionApi::expire_transaction(&client, &payment_response.transaction_id)?;
    println!("Transaction expired");

    // Get transactions by user ID
    let user_transactions = TransactionApi::get_transactions_by_user_id(&client, "user123")?;
    println!("User transactions: {:?}", user_transactions.transactions);

    // Search transactions
    let search_query = TransactionSearchQuery {
        status: Some("successful".to_string()),
        limit: Some(10),
        ..Default::default()
    };
    let search_results = TransactionApi::search_transactions(&client, &search_query)?;
    println!("Search results: {:?}", search_results.transactions);

    // Get service balance
    let balance = BalanceApi::get_service_balance(&client)?;
    println!("Service balance: {} {}", balance.balance, balance.currency);

    Ok(())
}
```

**To run the example:**

**Clone the repository:** `git clone https://github.com/yourusername/fapshi-sdk.git`
Replace `"your_api_user"` and `"your_api_key"` with your Fapshi credentials.
Run the example: 
```
cargo run --example create_payment
```

## Contributing
Contributions are welcome! Please submit issues or pull requests to the GitHub repository.

**Fork the repository.**
Create a feature branch: git checkout -b feature-name
Commit your changes: git commit -m "Add feature"
Push to the branch: git push origin feature-name
Open a pull request.

License
This project is licensed under the MIT License. See the LICENSE file for details.
Support
For questions about the Fapshi API, refer to documentation.fapshi.com or contact Fapshi support. For SDK-specific issues, open a GitHub issue.

Built with 💖 for the FAPSHI service.
