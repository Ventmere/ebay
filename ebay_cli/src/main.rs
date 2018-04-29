extern crate dotenv;
extern crate ebay;
extern crate serde_json;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate clap;

mod helpers;
mod sell;

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
    (@subcommand order =>
      (about: "Manage orders")
      (@subcommand get =>
        (about: "Retrieve the contents of an order based on its unique identifier")
        (@arg ID: +required "eBay order id")
      )
      (@subcommand get_recent_orders =>
        (about: "Retrieve recent orders")
      )
      (@subcommand get_fulfillments =>
        (about: "Retrieve the contents of all fulfillments currently defined for a specified order based on the order's unique identifier")
        (@arg ORDER_ID: +required "eBay order id")
      )
    )
  ).get_matches();

  dispatch! {
    matches =>
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

        (get_fulfillments =>
          (|m| {
            let order_id = m.value_of("ORDER_ID").unwrap();
            sell::order::get_fulfillments(order_id)
          })
        )
      )
  }
}
