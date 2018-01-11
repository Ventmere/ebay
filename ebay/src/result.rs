use reqwest::StatusCode;

error_chain! {
  errors {
    Request(path: String, status: StatusCode, body: String) {
      description("request error")
      display("request error: path = '{}', status = '{}', body = '{}'", path, status, body)
    }
  }

  foreign_links {
    Url(::url::ParseError);
    Http(::reqwest::Error);
  }
}

pub type EbayResult<T> = ::std::result::Result<T, Error>;