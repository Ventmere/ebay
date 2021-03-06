use chrono::{Duration, Utc};
use ebay::sell::fulfillment::{order::Filter, FulfillmentApi, GetOrdersParams};
use helpers::get_client;
use serde_json;
use std::io::stdout;

pub fn get_order(id: &str) {
  let order = get_client().get_order(id).unwrap();
  serde_json::to_writer_pretty(stdout(), &order).unwrap();
}

pub fn get_recent_orders() {
  let mut params = GetOrdersParams::default();
  params.filter = Filter::new()
    .created_after(Utc::now() - Duration::days(3))
    .into();
  let res = get_client().get_orders(&params).unwrap();
  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}

pub fn get_unshipped_orders() {
  let mut params = GetOrdersParams::default();
  params.filter = Filter::new().status_not_started_in_progress().into();
  let res = get_client().get_orders(&params).unwrap();
  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}

pub fn get_fulfillments(order_id: &str) {
  let res = get_client().get_shipping_fulfillments(order_id).unwrap();
  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}
