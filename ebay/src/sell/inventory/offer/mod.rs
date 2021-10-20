mod types;

pub use self::types::*;
use serde::{Serialize};

#[derive(Debug, Serialize, Default)]
pub struct GetOfferParams {
  pub offset: Option<i32>,
  pub limit: Option<usize>,
  pub format: Option<String>,
  pub marketplace_id: Option<String>,
  pub sku: String,
}
