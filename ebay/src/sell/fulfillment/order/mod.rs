use client::EbayClient;
use result::EbayResult;

mod types;

pub use self::types::*;
pub mod shipping_fulfillment;

#[derive(Serialize, Default)]
pub struct GetOrdersParams {
  pub offset: Option<i32>,
  pub limit: Option<usize>,
  pub filter: Option<Vec<String>>,
  #[serde(rename = "orderIds")]
  pub order_ids: Option<Vec<String>>,
}
