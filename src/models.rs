use serde::{Deserialize, Serialize};

/// Request payload for creating a payment link.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentRequest {
    /// The payment amount should be greater than 100.
    pub amount: f64,
    /// If the email is set, then the user will no longer be required to provide his/her email during the payment process
    pub email: Option<String>,
    /// URL to which your user will be redirected after completing a payment
    #[serde(rename = "redirectUrl")]
    pub redirect_url: Option<String>,
    /// The user ID associated with the payment (optional).
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "ExternalId")]
    /// This can be a transaction id, an order id or anything that can be used to reconcile this payment transaction to your application.
    pub external_id: Option<String>,
    /// Contains a message describing the reason for the payment.
    #[serde(rename = "message")]
    pub message: String,
    ///  If set to true, only international payment options will be available on the generated link
    #[serde(rename = "CardOnly")]
    pub card_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentTransactionResponse {
    #[serde(rename = "transId")]
    pub transaction_id: String,
}
/// Response payload for a created payment link.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentResponse {
    pub message: String,
    /// The generated payment link valid for 24hours.
    #[serde(rename = "link")]
    pub payment_link: String,
    /// The unique transaction ID.
    #[serde(rename = "transId")]
    pub transaction_id: String,
    #[serde(rename = "dateInitiated")]
    pub date_initiated: String,
}

/// Transaction status information.
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionStatus {
    #[serde(rename = "transId")]
    pub transaction_id: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue: Option<f64>,
    #[serde(rename = "payerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    pub email: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    #[serde(rename = "financialTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_transaction_id: Option<String>,
    #[serde(rename = "dateInitiated")]
    pub date_initiated: String,
    #[serde(rename = "dateConfirmed")]
    pub date_confirmed: String,
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

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "created" => Status::CREATED,
            "pending" => Status::PENDING,
            "successful" => Status::SUCCESSFUL,
            "failed" => Status::FAILED,
            "expired" => Status::EXPIRED,
            _ => Status::CREATED,
        }
    }
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
    /// The payment amount (integer >= 100).
    pub amount: f32, // Changed to i32 to enforce integer type
    /// The phone number to which the request will be performed (e.g., 67XXXXXXX).
    pub phone: String,
    /// The payment medium (either "mobile money" or "orange money").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>, // Optional, restricted to "mobile money" or "orange money"
    /// The name of the user performing the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // Optional
    /// The email of the user performing the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>, // Optional
    /// The user ID in the caller's system (1-100 characters, alphanumeric with -_).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>, // Optional
    /// The external ID for reconciliation (1-100 characters, alphanumeric with -_).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>, // Optional
    /// A message describing the reason for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>, // Optional, renamed from description
}
/// Response payload for a direct payment request.
#[derive(Serialize, Deserialize, Debug)]
pub struct DirectPaymentResponse {
    /// The unique transaction ID.
    #[serde(rename = "transId")]
    pub transaction_id: String,
    /// The status of the direct payment request.
    pub message: String,
    /// date initiated
    #[serde(rename = "dateInitiated")]
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
    #[serde(flatten)]
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
