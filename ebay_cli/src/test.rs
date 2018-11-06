use helpers::{get_client, Method};
use serde_json::{self, Value};
use std::io::stdout;

pub fn run() {
  let res: Value = get_client()
    .request(Method::Get, "/sell/inventory/v1/inventory_item")
    .unwrap()
    .send()
    .unwrap()
    .json()
    .unwrap();

  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}
