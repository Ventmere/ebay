use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Offers {
  pub total: i64,
  pub size: i64,
  pub href: String,
  pub limit: i64,
  pub offers: Vec<Offer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
  #[serde(rename = "offerId")]
  pub offer_id: String,
  pub sku: String,
  #[serde(rename = "marketplaceId")]
  pub marketplace_id: String,
  pub format: String,
  #[serde(rename = "availableQuantity")]
  pub available_quantity: i64,
  #[serde(rename = "categoryId")]
  pub category_id: String,
  pub listing: ListingDetails,
  #[serde(rename = "listingDescription")]
  pub listing_description: String,
  #[serde(rename = "listingPolicies")]
  pub listing_policies: ListingPolicies,
  #[serde(rename = "pricingSummary")]
  pub pricing_summary: PricingSummary,
  #[serde(rename = "quantityLimitPerBuyer")]
  pub quantity_limit_per_buyer: i64,
  pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingDetails {
  #[serde(rename = "listingId")]
  pub listing_id: String,
  #[serde(rename = "listingStatus")]
  pub listing_status: String,
  #[serde(rename = "soldQuantity")]
  pub sold_quantity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingPolicies {
  #[serde(rename = "fulfillmentPolicyId")]
  pub fulfillment_policy_id: String,
  #[serde(rename = "paymentPolicyId")]
  pub payment_policy_id: String,
  #[serde(rename = "returnPolicyId")]
  pub return_policy_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingSummary {
  pub price: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
  pub currency: String,
  pub value: String,
}
