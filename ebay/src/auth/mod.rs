mod grant;
mod exchange;
mod refresh;

pub use self::grant::{Grant, GrantUrl};
pub use self::exchange::{Exchange, ExchangeResponse};

#[derive(Debug, Clone)]
pub struct Credential {
  pub client_id: String,
  pub client_secret: String,
}