use ebay::trading::get_my_ebay_selling::ItemList;
use ebay::trading::types::ResponseMeta;
use ebay::trading::{Element, FromXmlElement, XmlResponse};

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

#[test]
fn test_response_errors_xml() {
  use ebay::result::EbayError;
  let src = include_str!("../../data/error.xml").to_string();
  let err = XmlResponse::<()>::parse_string(src).err().unwrap();
  if let EbayError::TradingApiResponseError(errs) = err {
    assert_eq!(format!("{:#?}", errs), r##"[
    Error {
        short_message: "Input data is invalid.",
        long_message: "Input data for tag <ActiveList[2].Include> is invalid or missing. Please check API documentation.",
        error_code: "37",
        error_classification: "RequestError",
        severity_code: "Error",
        error_parameters: [
            ErrorParameter {
                param_id: "0",
                value: "ActiveList[2].Include"
            }
        ]
    },
    Error {
        short_message: "Header \"X-EBAY-API-APP-NAME\" does not exist.",
        long_message: "Header \"X-EBAY-API-APP-NAME\" does not exist.",
        error_code: "10011",
        error_classification: "RequestError",
        severity_code: "Error",
        error_parameters: [
            ErrorParameter {
                param_id: "0",
                value: "X-EBAY-API-APP-NAME"
            }
        ]
    },
    Error {
        short_message: "Header \"X-EBAY-API-DEV-NAME\" does not exist.",
        long_message: "Header \"X-EBAY-API-DEV-NAME\" does not exist.",
        error_code: "10011",
        error_classification: "RequestError",
        severity_code: "Error",
        error_parameters: [
            ErrorParameter {
                param_id: "0",
                value: "X-EBAY-API-DEV-NAME"
            }
        ]
    },
    Error {
        short_message: "Header \"X-EBAY-API-CERT-NAME\" does not exist.",
        long_message: "Header \"X-EBAY-API-CERT-NAME\" does not exist.",
        error_code: "10011",
        error_classification: "RequestError",
        severity_code: "Error",
        error_parameters: [
            ErrorParameter {
                param_id: "0",
                value: "X-EBAY-API-CERT-NAME"
            }
        ]
    }
]"##)
  } else {
    panic!("unexpected error: {:?}", err)
  }
}
