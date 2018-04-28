use ebay::sell::fulfillment::FulfillmentApi;
use helpers::get_client;
use serde_json;
use std::io::stdout;

pub fn get_order(id: &str) {
  let order = get_client().get_order(id).unwrap();
  serde_json::to_writer_pretty(stdout(), &order).unwrap();
}

pub fn get_fulfillment(id: &str) {}
