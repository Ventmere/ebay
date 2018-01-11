#[doc(hidden)]
#[macro_export]
macro_rules! check_resp {
  ($url:expr, $resp:expr) => ({
    use result::ErrorKind;
    use reqwest::StatusCode;

    if $resp.status() != StatusCode::Ok {
      let body = $resp.text()?;
      return Err(
        ErrorKind::Request($url.to_owned(), $resp.status(), body).into(),
      );
    }
  })
}