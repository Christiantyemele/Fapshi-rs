use serde::{Deserialize, Serialize};

/// Request payload for creating a payment link.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentRequest {
    /// The payment amount should be greater than 100.
    pub amount: f64,
    /// A description of the payment.
    pub description: String,
    /// If the email is set, then the user will no longer be required to provide his/her email during the payment process
    pub email: Option<String>,
    /// The customer's email address (optional).
    pub customer_email: Option<String>,
    /// This can be a transaction id, an order id or anything that can be used to reconcile this payment transaction to your application.
    pub external_id: Option<String>,
    /// The user ID associated with the payment (optional).
    pub user_id: Option<String>,
    /// URL to which your user will be redirected after completing a payment
    pub redirect_url: Option<String>,
}

/// Response payload for a created payment link.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentResponse {
    /// The generated payment link valid for 24hours.
    pub payment_link: String,
    /// The unique transaction ID.
    pub transaction_id: String,
}

/// Transaction status information.
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionStatus {
    /// The unique transaction ID.
    pub transaction_id: String,
    /// The current status
    pub status: Status,
    /// The transaction amount.
    pub amount: f64,
    /// revenue
    pub revenue: f64,
    /// date initiated
    pub date_initiated: String,
    /// date confirmed
    pub date_confirmed: String,
    /// The payment medium (e.g., "mobile money", "orange money", "card").
    medium: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
    CREATED,
    PENDING,
    SUCCESSFUL,
    FAILED,
    EXPIRED,
}

/// Expired Transaction
pub struct ExpiredTransaction {
    /// The unique transaction ID.
    pub transaction_id: String,
}

pub struct ExpiredTransactionResponse {
    /// The status of the expired transaction.
    pub status: String,
    /// The unique transaction ID.
    pub transaction_id: String,
    /// The transaction amount.
    pub amount: f64,
}

/// Configuration for a webhook.
#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookConfig {
    /// The URL to receive webhook notifications.
    pub url: String,
    /// The service ID for the webhook.
    pub service_id: String,
}

/// Request payload for initiating a direct payment to a mobile device.
#[derive(Serialize, Deserialize, Debug)]
pub struct DirectPaymentRequest {
    /// The payment amount (amount >= 100).
    pub amount: f64,
    /// The currency code (e.g., "USD").
    pub currency: String,
    /// The phone number to receive the payment prompt.
    pub phone_number: String,
    /// A description of the payment.
    pub description: String,
}

/// Response payload for a direct payment request.
#[derive(Serialize, Deserialize, Debug)]
pub struct DirectPaymentResponse {
    /// The unique transaction ID.
    pub transaction_id: String,
    /// The status of the direct payment request.
    pub status: String,
    /// date initiated
    pub date_initiated: String,
}

/// Query parameters for searching transactions.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TransactionSearchQuery {
    /// Transaction status (e.g., "created", "successful", "failed", "expired").
    pub status: Status,
    /// Payment medium (e.g., "mobile money", "orange money").
    pub medium: String,
    /// Name of the user performing the payment (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Start date (yyyy-mm-dd) for transactions.
    pub start: String,
    /// End date (yyyy-mm-dd) for transactions.
    pub end: String,
    /// Exact transaction amount.
    pub amt: f64,
    /// Maximum number of transactions to return
    pub limit: u32,
}

/// Response payload for a transaction search or user ID query.
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionList {
    /// Array of transactions matching the query.
    pub transactions: Vec<TransactionStatus>,
}

/// Response payload for service balance.
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceBalance {
    /// The current balance amount.
    pub balance: f64,
    /// The currency code.
    pub currency: String,
}

pub struct Payouts {
    /// amount to be sent to the user.
    pub amount: f64,
    /// phone number to which the amount will be sent e.g., 67XXXXXXX, 69XXXXXXX, 65XXXXXXX.
    pub phone_number: String,
    /// medium can either be “mobile money” for MTN numbers or “orange money” for Orange numbers.
    pub medium: String,
    /// name of the user receiving the payment.
    pub name: Option<String>,
    /// email of the user receiving the payment
    pub email: Option<String>,
    /// contains a message describing the reason for the payout.
    pub message: Option<String>,
}
