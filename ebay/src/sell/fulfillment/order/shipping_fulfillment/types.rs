use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ShippingFulfillment {
  #[serde(rename = "fulfillmentId")]
  pub fulfillment_id: String,
  #[serde(rename = "shipmentTrackingNumber")]
  pub shipment_tracking_number: Option<String>,
  #[serde(rename = "shippingCarrierCode")]
  pub shipping_carrier_code: Option<String>,
  #[serde(rename = "shippingServiceCode")]
  pub shipping_service_code: Option<String>,
  #[serde(rename = "shippedDate")]
  pub shipped_date: Option<DateTime<Utc>>,
  #[serde(rename = "lineItems")]
  pub line_items: Option<Vec<LineItemReference>>,
}

#[derive(Serialize, Deserialize)]
pub struct LineItemReference {
  /// The globally unique eBay-generated identifier of the line item.
  /// Note: A single line item can consist of multiple units of a purchased item,
  /// and one unit can consist of multiple parts or components. Although these
  /// components might be provided by the manufacturer in separate
  /// packaging, the seller cannot distribute them among multiple shipping
  /// packages. Thus, each line item ID will appear in exactly one fulfillment.
  #[serde(rename = "lineItemId")]
  pub line_item_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingFulfillmentPagedCollection {
  pub fulfillments: Vec<ShippingFulfillment>,
  pub total: i32,
  pub warnings: Option<Vec<Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingFulfillmentDetails {
  #[serde(rename = "trackingNumber")]
  pub tracking_number: Option<String>,
  #[serde(rename = "shippingCarrierCode")]
  pub shipping_carrier_code: Option<String>,
  #[serde(rename = "shippingServiceCode")]
  pub shipping_service_code: Option<String>,
  #[serde(rename = "shippedDate")]
  pub shipped_date: Option<DateTime<Utc>>,
  #[serde(rename = "lineItems")]
  pub line_items: Option<Vec<LineItemReference>>,
}