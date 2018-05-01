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

impl ShippingFulfillmentDetails {
  pub fn new(carrier_code: &str, tracking: &str) -> ShippingFulfillmentDetailsBuilder {
    ShippingFulfillmentDetailsBuilder {
      inner: ShippingFulfillmentDetails {
        tracking_number: tracking.to_owned().into(),
        shipping_carrier_code: carrier_code.to_owned().into(),
        shipping_service_code: None,
        shipped_date: Some(Utc::now()),
        line_items: None,
      },
    }
  }
}

pub struct ShippingFulfillmentDetailsBuilder {
  inner: ShippingFulfillmentDetails,
}

impl ShippingFulfillmentDetailsBuilder {
  pub fn shipping_service_code(&mut self, code: &str) -> &mut Self {
    self.inner.shipping_service_code = code.to_owned().into();
    self
  }

  pub fn shipped_date(&mut self, date: DateTime<Utc>) -> &mut Self {
    self.inner.shipped_date = date.into();
    self
  }

  pub fn add_item(&mut self, line_item_id: &str) -> &mut Self {
    self
      .inner
      .line_items
      .get_or_insert_with(|| vec![])
      .push(LineItemReference {
        line_item_id: line_item_id.to_owned(),
      });
    self
  }

  pub fn finalize(&mut self) -> ShippingFulfillmentDetails {
    let replace = ShippingFulfillmentDetails {
      tracking_number: self.inner.tracking_number.clone(),
      shipping_carrier_code: self.inner.shipping_carrier_code.clone(),
      shipping_service_code: None,
      shipped_date: Some(Utc::now()),
      line_items: None,
    };
    ::std::mem::replace(&mut self.inner, replace)
  }
}
