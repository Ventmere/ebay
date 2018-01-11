use ebay::auth::Exchange;
use get_env;
use HTTP_CLIENT;

#[test]
fn exchange() {
  let env = get_env();
  let ex = Exchange {
    host: &env.host,
    credential: &env.credential,
    ru_name: &env.ru_name,
  };

  let res = ex.exchange(&HTTP_CLIENT, &env.code).unwrap();

  println!("{:#?}", res);
}