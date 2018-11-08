use ebay::trading::Xml;
use helpers::get_client;

pub fn run() {
  // use std::io::stdout;
  // let res: Value = get_client()
  //   .request(Method::Get, "/sell/inventory/v1/inventory_item")
  //   .unwrap()
  //   .send()
  //   .unwrap()
  //   .json()
  //   .unwrap();

  // serde_json::to_writer_pretty(stdout(), &res).unwrap();
  run_get_active_listing()
}

fn run_get_active_listing() {
  let client = get_client();
  let res: Xml<()> = client
    .request_trading_api(
      "GetMyeBaySelling",
      vec![ebay_xml_element!{
        ActiveList[][
          Include[][true]
          Pagination[][
            EntriesPerPage[][200]
            PageNumber[][1]
          ]
        ]
      }],
    )
    .unwrap();
  println!("{}", res.text())
}
