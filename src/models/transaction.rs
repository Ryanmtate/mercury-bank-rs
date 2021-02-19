use super::{
    AddressWithoutName, DomesticWireRoutingInfoRaw, ElectronicRoutingInfoRaw,
    InternationalWireRoutingInfoRaw,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "bankDescription", skip_serializing_if = "Option::is_none")]
    pub bank_description: Option<String>,
    #[serde(rename = "counterpartyId", skip_serializing_if = "Option::is_none")]
    pub counterparty_id: Option<String>,
    #[serde(rename = "counterpartyName", skip_serializing_if = "Option::is_none")]
    pub counterparty_name: Option<String>,
    #[serde(
        rename = "counterpartyNickname",
        skip_serializing_if = "Option::is_none"
    )]
    pub counterparty_nickname: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dashboardLink", skip_serializing_if = "Option::is_none")]
    pub dashboard_link: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<TransactionDetails>,
    #[serde(
        rename = "estimatedDeliveryDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_delivery_date: Option<String>,
    #[serde(rename = "failedAt", skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>, //"externalTransfer" | "internalTransfer" | "outgoingPayment" | "debitCardTransaction" | "incomingDomesticWire" | "checkDeposit" | "incomingInternationalWire" | "fee" | "other",
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "externalMemo", skip_serializing_if = "Option::is_none")]
    pub external_memo: Option<String>,
    #[serde(rename = "postedAt", skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
    #[serde(rename = "reasonForFailure", skip_serializing_if = "Option::is_none")]
    pub reason_for_failure: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>, //"pending" | "sent" | "cancelled" | "failed",
    #[serde(rename = "feeId", skip_serializing_if = "Option::is_none")]
    pub fee_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionDetails {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressWithoutName>,
    #[serde(
        rename = "domesticWireRoutingInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub domestic_wire_routing_info: Option<DomesticWireRoutingInfoRaw>,
    #[serde(
        rename = "electronicRoutingInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub electronic_routing_info: Option<ElectronicRoutingInfoRaw>,
    #[serde(
        rename = "electronicRoutingInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub international_wire_routing_info: Option<InternationalWireRoutingInfoRaw>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transactions {
    pub total: u64,
    pub transactions: Vec<Transaction>,
}
