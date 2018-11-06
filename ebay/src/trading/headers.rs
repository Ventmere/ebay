use reqwest::header::Headers;
use reqwest::RequestBuilder;

const EBAY_API_COMPATIBILITY_LEVEL: &'static str = "967";
const EBAY_API_SITEID: &'static str = "0";

pub trait AddTradingApiHeaders {
  fn add_trading_api_headers(&mut self, call_name: &str) -> &mut Self;
}

impl AddTradingApiHeaders for RequestBuilder {
  fn add_trading_api_headers(&mut self, call_name: &str) -> &mut Self {
    let mut headers = Headers::new();
    headers.set_raw(
      "X-EBAY-API-COMPATIBILITY-LEVEL",
      EBAY_API_COMPATIBILITY_LEVEL,
    );
    headers.set_raw("X-EBAY-API-CALL-NAME", call_name.to_owned());
    headers.set_raw("X-EBAY-API-SITEID", EBAY_API_SITEID);
    headers.set_raw("Accept", "xml");
    headers.set_raw("Content-Type", "application/xml");
    self.headers(headers)
  }
}
