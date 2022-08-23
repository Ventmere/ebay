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
    .request_trading_api::<ebay::trading::get_my_ebay_selling::Response>("GetMyeBaySelling", build_active_list_elements(1))
    .unwrap();
  serde_json::to_writer_pretty(stdout(), &res.into_inner()).unwrap();
}

pub fn get_item_quantity_by_item_id(item_id: &str) {
  let res = get_client()
    .request_trading_api::<Option<Element>>("GetItem", vec![ebay_xml_element![
      ItemID[][item_id]
    ]])
    .unwrap()
    .into_inner();
  if let Some(elem) = res {
    println!("{:?}", elem.get_child("Item").unwrap().get_child("Quantity").unwrap().text);
    elem.write(stdout()).unwrap();
  }
}

pub fn set_item_quantity_by_item_id(item_id: &str, quantity: i32) {
  let elem = ebay_xml_element![
    InventoryStatus[][
      ItemID[][item_id]
      Quantity[][quantity]
    ]
  ];
  elem.write(std::io::stdout()).unwrap();
  get_client()
    .request_trading_api::<Option<Element>>("ReviseInventoryStatus", vec![elem])
    .unwrap();
}

pub fn set_mv_item_quantity_by_item_id(item_id: &str, sku: &str, quantity: i32) {
  let elem = ebay_xml_element![
    InventoryStatus[][
      ItemID[][item_id]
      SKU[][sku]
      Quantity[][quantity]
    ]
  ];
  elem.write(std::io::stdout()).unwrap();
  get_client()
    .request_trading_api::<Option<Element>>("ReviseInventoryStatus", vec![elem])
    .unwrap();
}