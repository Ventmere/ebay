//! Fulfillment API
//! [Doc](https://developer.ebay.com/api-docs/sell/fulfillment/overview.html)

use client::*;
use result::EbayResult;

pub mod order;

pub use self::order::{GetOrdersParams, Order, OrderSearchPagedCollection};

pub trait FulfillmentApi {
  fn get_orders(&self, params: &GetOrdersParams) -> EbayResult<OrderSearchPagedCollection>;
  fn get_order(&self, id: &str) -> EbayResult<Order>;
}

impl FulfillmentApi for EbayClient {
  fn get_orders(&self, params: &GetOrdersParams) -> EbayResult<OrderSearchPagedCollection> {
    self
      .request(Method::Get, "/sell/fulfillment/v1/order")?
      .query(params)
      .send()?
      .get_response()
  }

  fn get_order(&self, id: &str) -> EbayResult<Order> {
    self
      .request(Method::Get, &format!("/sell/fulfillment/v1/order/{}", id))?
      .send()?
      .get_response()
  }
}
