//! Fulfillment API
//! [Doc](https://developer.ebay.com/api-docs/sell/fulfillment/overview.html)

use crate::{client::*, result::EbayError};
use crate::result::EbayResult;

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
    let mut b = self.request(Method::GET, "/sell/fulfillment/v1/order")?;
    b = b.query(params);
    if let Some(ref ids) = params.order_ids {
      b = b.query(&[("orderIds", ids.join(",") as String)]);
    }
    b.send()?.get_response()
  }

  fn get_order(&self, id: &str) -> EbayResult<Order> {
    self
      .request(Method::GET, &format!("/sell/fulfillment/v1/order/{}", id))?
      .send()?
      .get_response()
  }

  fn get_shipping_fulfillments(
    &self,
    order_id: &str,
  ) -> EbayResult<ShippingFulfillmentPagedCollection> {
    self
      .request(
        Method::GET,
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
        Method::GET,
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
    let res = self
      .request(
        Method::POST,
        &format!(
          "/sell/fulfillment/v1/order/{}/shipping_fulfillment",
          order_id
        ),
      )?
      .json(details)
      .send()?;

    let res = res.error_for_status()?;

    let location = res
      .headers()
      .get("location")
      .and_then(|location| location.to_str().ok()?.split('/').last().map(str::to_string))
      .ok_or_else(|| EbayError::Msg("Location header was not found".to_owned()))?;

    Ok(location.to_owned())
  }
}
