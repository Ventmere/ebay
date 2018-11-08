use ebay::trading::get_my_ebay_selling::ItemList;
use ebay::trading::types::ResponseMeta;
use ebay::trading::{Element, FromXmlElement};

#[test]
fn test_listings_response_from_xml() {
  use std::io::Cursor;
  let src = Cursor::new(include_bytes!("../../data/listings.xml") as &[u8]);
  let elem = Element::parse(src).unwrap();
  let res = ResponseMeta::from_xml_element(&elem).unwrap();
  assert_eq!(res.timestamp, "2018-10-18T04:51:47.320Z");
  assert_eq!(res.ack, "Success");
  assert_eq!(res.version, "1083");
  assert_eq!(res.build, "E1083_CORE_APISELLING_18856597_R1");

  let list_elem = elem.get_child("ActiveList").unwrap();
  let list = ItemList::from_xml_element(list_elem).unwrap();
  println!("{:#?}", list)
}
