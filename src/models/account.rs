#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "availableBalance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<f64>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "currentBalance", skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<f64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingNumber", skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>, // "active" | "deleted" | "pending",
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>, // "mercury" | "external" | "recipient",
    #[serde(
        rename = "canReceiveTransactions",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_receive_transactions: Option<bool>, // Whether an account can receive transactions, based on the information from our partner bank
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "legalBusinessName", skip_serializing_if = "Option::is_none")]
    pub legal_business_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}
