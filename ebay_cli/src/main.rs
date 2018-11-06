extern crate chrono;
extern crate dotenv;
extern crate ebay;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;

mod helpers;
mod sell;
mod test;

macro_rules! dispatch {
  ($matches:expr => $head:tt $($rest:tt)*) => {
    dispatch!(ITEM $matches, $head);
    dispatch!($matches => $($rest)*);
  };

  ($matches:expr => ) => {};

  (ITEM $matches:expr, ($handler:expr)) => {
    ($handler as fn(&clap::ArgMatches))(&$matches)
  };

  (ITEM $matches:expr, ($cmd:ident => $($sub:tt)+)) => {
    if let Some(matches) = $matches.subcommand_matches(stringify!($cmd)) {
      dispatch!(matches => $($sub)*);
    }
  };
}

fn main() {
  let matches = clap_app!(myapp =>
    (@subcommand test => )
    (@subcommand order =>
      (about: "Manage orders")
      (@subcommand get =>
        (about: "Retrieve the contents of an order based on its unique identifier")
        (@arg ID: +required "eBay order id")
      )
      (@subcommand get_recent_orders =>
        (about: "Retrieve recent orders")
      )
      (@subcommand get_unshipped_orders =>
        (about: "Retrieve unshipped orders")
      )
      (@subcommand get_fulfillments =>
        (about: "Retrieve the contents of all fulfillments currently defined for a specified order based on the order's unique identifier")
        (@arg ORDER_ID: +required "eBay order id")
      )
      (@subcommand ship =>
        (@arg ORDER_ID: +required "eBay order id")
        (@arg LINE_ITEM_ID: +required "Line item id")
        (@arg CARRIER: -c --cariier +required +takes_value "Carrier")
        (@arg TRACKING: -t --tracking +required +takes_value "Tracking number")        
      )
    )
    (@subcommand inventory =>
      (@subcommand bulk_migrate_listing =>
        (@arg LISTING_IDS: -i ... +takes_value "Add listing id")
      )
      (@subcommand get_inventory_locations =>
        (about: "Retrieves all defined details for every inventory location associated with the seller's account")
      )
      (@subcommand get_inventory_items =>
        (about: "Retrieves all inventory item records defined for the seller's account")
      )
      (@subcommand get_offers =>
        (about: "Retrieves all existing offers for the specified SKU value")
        (@arg SKU: +required "eBay SKU")
      )
    )
  ).get_matches();

  dispatch! {
    matches =>
      (test =>
        (|_| {
          test::run()
        })
      )
      (order =>
        (get =>
          (|m| {
            let id = m.value_of("ID").unwrap();
            sell::order::get_order(id)
          })
        )

        (get_recent_orders =>
          (|_| {
            sell::order::get_recent_orders()
          })
        )

        (get_unshipped_orders =>
          (|_| {
            sell::order::get_unshipped_orders()
          })
        )

        (get_fulfillments =>
          (|m| {
            let order_id = m.value_of("ORDER_ID").unwrap();
            sell::order::get_fulfillments(order_id)
          })
        )

        (ship =>
          (|m| {
            use ebay::sell::fulfillment::*;
            let client = helpers::get_client();
            let order_id = m.value_of("ORDER_ID").unwrap();
            let line_item_id = m.value_of("LINE_ITEM_ID").unwrap();
            let carrier = m.value_of("CARRIER").unwrap();
            let tracking = m.value_of("TRACKING").unwrap();
            let shipment = ShippingFulfillmentDetails::new(carrier, tracking)
              .add_item(line_item_id)
              .finalize();

            println!("Request:");
            helpers::dump_json(&shipment);

            println!("\nResponse:");
            let res = client.create_shipping_fulfillment(order_id, &shipment).unwrap();
            helpers::dump_json(res)
          })
        )
      )
      (inventory =>
        (migrate =>
          (|_| {
            sell::inventory::get_inventory_locations()
          })
        )

        (get_inventory_locations =>
          (|_| {
            sell::inventory::get_inventory_locations()
          })
        )

        (get_inventory_items =>
          (|_| {
            sell::inventory::get_inventory_items()
          })
        )

        (get_offers =>
          (|m| {
            let sku = m.value_of("SKU").unwrap();
            sell::inventory::get_offers(&sku)
          })
        )

        (bulk_migrate_listing =>
          (|m| {
            let ids: Vec<_> = m.values_of("LISTING_IDS").expect("No listing ids (-i)").collect();
            sell::inventory::bulk_migrate_listing(&ids);
          })
        )
      )
  }
}
