//! Fulfillment API
//! [Doc](https://developer.ebay.com/api-docs/sell/fulfillment/overview.html)

use client::*;
use result::EbayResult;

pub mod order;

pub use self::order::{
  shipping_fulfillment::{
    ShippingFulfillment, ShippingFulfillmentDetails, ShippingFulfillmentDetailsBuilder,
    ShippingFulfillmentPagedCollection,
  },
  GetOrdersParams, Order, OrderSearchPagedCollection,
};

pub trait FulfillmentApi {
  fn get_orders(&self, params: &GetOrdersParams) -> EbayResult<OrderSearchPagedCollection>;
  fn get_order(&self, id: &str) -> EbayResult<Order>;
  fn get_shipping_fulfillments(
    &self,
    order_id: &str,
  ) -> EbayResult<ShippingFulfillmentPagedCollection>;
  fn get_shipping_fulfillment(&self, order_id: &str, id: &str) -> EbayResult<ShippingFulfillment>;
  fn create_shipping_fulfillment(
    &self,
    order_id: &str,
    details: &ShippingFulfillmentDetails,
  ) -> EbayResult<String>;
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

  fn get_shipping_fulfillments(
    &self,
    order_id: &str,
  ) -> EbayResult<ShippingFulfillmentPagedCollection> {
    self
      .request(
        Method::Get,
        &format!(
          "/sell/fulfillment/v1/order/{}/shipping_fulfillment",
          order_id
        ),
      )?
      .send()?
      .get_response()
  }

  fn get_shipping_fulfillment(&self, order_id: &str, id: &str) -> EbayResult<ShippingFulfillment> {
    self
      .request(
        Method::Get,
        &format!(
          "/sell/fulfillment/v1/order/{}/shipping_fulfillment/{}",
          order_id, id
        ),
      )?
      .send()?
      .get_response()
  }

  fn create_shipping_fulfillment(
    &self,
    order_id: &str,
    details: &ShippingFulfillmentDetails,
  ) -> EbayResult<String> {
    use reqwest::header::Location;
    let mut res = self
      .request(
        Method::Post,
        &format!(
          "/sell/fulfillment/v1/order/{}/shipping_fulfillment",
          order_id
        ),
      )?
      .json(details)
      .send()?;

    check_resp!(res);

    let location = res
      .headers()
      .get::<Location>()
      .and_then(|location| location.split('/').last().map(str::to_string))
      .ok_or_else(|| "Location header was not found".to_owned())?;

    Ok(location.to_owned())
  }
}
