use ebay::trading::get_my_ebay_selling::Response;
use ebay::trading::XmlResponse;

#[test]
fn test_listings_response_from_xml() {
  let src = include_str!("../../data/listings.xml").to_string();
  let res = XmlResponse::<Response>::parse_string(src).unwrap();
  println!("{:#?}", res.into_inner())
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
