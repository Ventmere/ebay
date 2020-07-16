use ebay::trading::get_my_ebay_selling::Response;
use ebay::trading::Element;
use helpers::get_client;
use std::io::stdout;

pub fn build_active_list_elements(page: i64) -> Vec<Element> {
  let elem = ebay_xml_element![
    ActiveList[][
      Include[][true]
      Pagination[][
        EntriesPerPage[][200]
        PageNumber[][page]
      ]
    ]
  ];
  vec![elem]
}

pub fn get_my_ebay_selling() {
  let res = get_client()
    .request_trading_api::<Response>("GetMyeBaySelling", build_active_list_elements(1))
    .unwrap();
  serde_json::to_writer_pretty(stdout(), &res.into_inner()).unwrap();
}
