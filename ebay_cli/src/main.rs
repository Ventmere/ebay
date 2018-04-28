extern crate dotenv;
extern crate ebay;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
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
      (@subcommand get_fulfillment =>
        (about: "Retrieve the contents of a fulfillment based on its unique identifier")
        (@arg ID: +required "eBay fulfillment id")
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

        (get_fulfillment =>
          (|m| {
            let id = m.value_of("ID").unwrap();
            println!("id = {}", id);
          })
        )
      )
  }
}
