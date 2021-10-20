mod types;

pub use self::types::*;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct GetInventoryItemsParams {
  pub offset: Option<i32>,
  pub limit: Option<usize>,
}
