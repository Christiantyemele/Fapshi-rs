use fapshi_rs::{
    api::{balance::BalanceApi, payment::PaymentApi, transaction::TransactionApi},
    client::FapshiClient,
    models::{DirectPaymentRequest, PaymentRequest},
};

/// Example demonstrating how to use the Fapshi SDK for various API operations.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client in sandbox mode
    let client = FapshiClient::new(
        "your_apiuser", // cf58da14-1daa-4d48-a5c6-9033d479fcc1
        "your_apikey",  //  "FAK_TEST_74f26535e703330ebd13",
        true,
    )?;

    // Create a payment link
    let payment_request = PaymentRequest {
        amount: 100.0,
        email: Some("wilfouang@gmail.com".to_string()),
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
        phone: "654988322".to_string(),
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
