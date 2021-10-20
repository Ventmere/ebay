use crate::client::EbayClient;
use reqwest::Method;
use crate::result::*;
pub use xmltree::Element;

const ENDPOINT: &'static str = "https://api.ebay.com/ws/api.dll";

#[macro_use]
mod xml_helper;
mod headers;
pub mod types;

pub use self::xml_helper::{FromXmlElement, XmlResponse};

pub mod get_my_ebay_selling;

impl EbayClient {
  pub fn request_trading_api<T: FromXmlElement>(
    &self,
    call_name: &str,
    request_elements: Vec<Element>,
  ) -> EbayResult<XmlResponse<T>> {
    use self::headers::AddTradingApiHeaders;
    let res = self
      .http
      .request(Method::POST, ENDPOINT)
      .add_trading_api_headers(call_name)
      .body(wrap_request_elements(
        call_name,
        &self.trading_api_token,
        request_elements,
      )?)
      .send()?;

    XmlResponse::parse(res)
  }
}

fn wrap_request_elements(
  call_name: &str,
  auth_token: &str,
  mut elements: Vec<Element>,
) -> EbayResult<String> {
  use std::io::Cursor;

  let mut elem = ebay_xml_element!(
    Request[xmlns="urn:ebay:apis:eBLBaseComponents"][
      RequesterCredentials[][
        eBayAuthToken[][
          auth_token
        ]
      ]
    ]
  );
  elem.name = format!("{}Request", call_name);
  elem.children.append(&mut elements);
  let mut buf: Cursor<Vec<u8>> = Cursor::new(vec![]);
  elem.write(&mut buf)?;
  Ok(::std::str::from_utf8(&buf.into_inner())?.to_string())
}
