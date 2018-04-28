use reqwest::{Response, StatusCode};
use result::{EbayResult, ErrorKind};
use serde::Deserialize;
use serde_json;

#[doc(hidden)]
#[macro_export]
macro_rules! check_resp {
  ($url:expr, $resp:expr) => {{
    use reqwest::StatusCode;
    use result::ErrorKind;

    if $resp.status() != StatusCode::Ok {
      let body = $resp.text()?;
      return Err(ErrorKind::Request($url.to_owned(), $resp.status(), body).into());
    }
  }};
}

pub fn read_ebay_response<T: for<'de> Deserialize<'de>>(resp: &mut Response) -> EbayResult<T> {
  let body = resp.text()?;

  if resp.status() != StatusCode::Ok {
    return Err(ErrorKind::Request(resp.url().to_string(), resp.status(), body).into());
  }

  match serde_json::from_str(&body) {
    Ok(v) => Ok(v),
    Err(err) => return Err(ErrorKind::Deserialize(err.to_string(), body).into()),
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! uppercase_str_enum {
  (pub enum $name:ident { $($v:ident,)+ }) => {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum $name {
      $(
        $v
      ),*
    }
  };
}
