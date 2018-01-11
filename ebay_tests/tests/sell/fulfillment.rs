use ebay::{client::Method, sell::fulfillment::order};
use get_client;
use serde_json::{self, Value};
use std::{fs, io::prelude::*, thread::sleep, time::Duration};

#[derive(Serialize, Deserialize)]
struct Res {
  pub href: String,
  pub limit: i32,
  pub next: Option<String>,
  pub offset: i32,
  pub orders: Vec<Value>,
  pub prev: Option<String>,
  pub total: i32,
  pub warnings: Option<Vec<Value>>,
}

#[test]
fn get_all_orders() {
  let client = get_client();

  let mut offset = 0;

  loop {
    println!("downloading orders, offset = {}", offset);

    let mut b = client
      .request(Method::Get, "/sell/fulfillment/v1/order")
      .unwrap();

    let mut res = b.query(&[("limit", "1000"), ("offset", &offset.to_string())])
      .send()
      .unwrap();

    let res: Res = res.json().unwrap();

    println!("len = {}", res.orders.len());

    {
      let f = fs::File::create(format!("data/orders_{}.json", offset)).unwrap();
      serde_json::to_writer_pretty(f, &res).unwrap();
    }

    if res.orders.is_empty() {
      break;
    }

    offset = offset + res.orders.len();
    sleep(Duration::from_secs(1));
  }
}

#[test]
fn test_all_orders() {
  let data: Res = serde_json::from_reader(fs::File::open("data/orders_0.json").unwrap()).unwrap();

  println!("len = {}", data.orders.len());

  for (i, v) in data.orders.into_iter().enumerate() {
    println!("testing {}", i);

    let text = serde_json::to_string_pretty(&v).unwrap();
    write!(
      &mut fs::File::create("data/order.json").unwrap(),
      "{}",
      text
    ).unwrap();

    serde_json::from_str::<order::Order>(&text).unwrap();
  }
}
