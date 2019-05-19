use get_env;
use HTTP_CLIENT;

const SCOPES: [&'static str; 10] = [
  "https://api.ebay.com/oauth/api_scope",
  "https://api.ebay.com/oauth/api_scope/sell.marketing.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.marketing",
  "https://api.ebay.com/oauth/api_scope/sell.inventory.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.inventory",
  "https://api.ebay.com/oauth/api_scope/sell.account.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.account",
  "https://api.ebay.com/oauth/api_scope/sell.fulfillment.readonly",
  "https://api.ebay.com/oauth/api_scope/sell.fulfillment",
  "https://api.ebay.com/oauth/api_scope/sell.analytics.readonly",
];

#[test]
#[ignore]
fn grant() {
  use ebay::auth::Grant;

  let env = get_env();
  let g = Grant {
    credential: &env.credential,
    ru_name: &env.ru_name,
    scopes: &SCOPES,
  };

  let url = g.build_grant_url("test").unwrap();

  ::os_open_url(&url.to_string())
}

#[test]
#[ignore]
fn exchange() {
  use ebay::auth::Exchange;

  let env = get_env();
  let ex = Exchange {
    credential: &env.credential,
    ru_name: &env.ru_name,
  };

  let res = ex.exchange(&HTTP_CLIENT, &env.code).unwrap();

  println!("{:#?}", res);
}

#[test]
#[ignore]
fn refresh() {
  use ebay::auth::Refresh;

  let env = get_env();
  let r = Refresh {
    credential: &env.credential,
    scopes: &SCOPES,
  };

  let res = r.refresh(&HTTP_CLIENT, &env.refresh_token).unwrap();

  println!("{:#?}", res);
}
