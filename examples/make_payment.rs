use fapshi_rs::{
    api::{balance::BalanceApi, payment::PaymentApi, transaction::TransactionApi},
    client::FapshiClient,
    models::{DirectPaymentRequest, PaymentRequest},
};

/// Example demonstrating how to use the Fapshi SDK for various API operations.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client in sandbox mode
    let client = FapshiClient::new("cf58da14-1daa-4d48-a5c6-", "9033d479fcc1", true)?;

    // Create a payment link
    let payment_request = PaymentRequest {
        amount: 100.0,
        message: "Test payment".to_string(),
        user_id: Some("user123".to_string()),
        redirect_url: None,
        card_only: None,
        email: Some("test@gmail.com".to_string()),
        external_id: Some("order123".to_string()),
    };
    let payment_response = PaymentApi::create_payment(&client, &payment_request)?;
    println!("Payment link: {}", payment_response.payment_link);
    println!("Transaction ID: {}", payment_response.transaction_id);

    // Initiate a direct payment
    let direct_request = DirectPaymentRequest {
        amount: 50.0,
        currency: "USD".to_string(),
        phone_number: "654988322".to_string(),
        description: "Direct payment test".to_string(),
    };
    let direct_response = PaymentApi::initiate_direct_payment(&client, &direct_request)?;
    println!(
        "Direct Payment Transaction ID: {}",
        direct_response.transaction_id
    );

    // Expire a transaction
    TransactionApi::expire_transaction(&client, &payment_response.transaction_id)?;
    println!("Transaction {} expired", payment_response.transaction_id);

    // Get transactions by user ID
    let user_transactions = TransactionApi::get_transactions_by_user_id(&client, "user123")?;
    println!("User transactions: {:?}", user_transactions.transactions);

    // Get service balance
    let balance = BalanceApi::get_service_balance(&client)?;
    println!("Service balance: {} {}", balance.balance, balance.currency);

    Ok(())
}
