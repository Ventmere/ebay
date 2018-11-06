use client::EbayClient;
use reqwest::Method;
use result::*;
use xmltree::Element;

// const ENDPOINT: &'static str = "https://api.ebay.com/ws/api.dll";

#[macro_use]
mod xml_helper;
mod headers;
mod listing;

// impl EbayClient {
//   pub fn request_trading_api(&self, call_name: &str, request_xml: String) -> EbayResult<Element> {
//     let mut b = self
//       .http
//       .request(Method::Post, ENDPOINT)
//       .header("X-EBAY-API-COMPATIBILITY-LEVEL", 967);
//     unimplemented!()
//   }
// }

// pub trait TradingApi {
//   /// GetMyeBaySelling
//   fn get_my_ebay_selling(&self);
// }

// fn wrap_request_elements(call_name: &str, auth_token: &str, mut elements: Vec<Element>) -> Element {
//   let mut elem = xml_element!(
//     Request[xmlns="urn:ebay:apis:eBLBaseComponents"][
//       RequesterCredentials[][
//         eBayAuthToken[][
//           auth_token
//         ]
//       ]
//     ]
//   );
//   elem.name = format!("{}Request", call_name);
//   elem.children.append(&mut elements);
//   elem
// }
