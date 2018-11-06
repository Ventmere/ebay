mod types;

pub use self::types::*;

#[derive(Debug, Serialize, Default)]
pub struct GetLocationParams {
  pub offset: Option<i32>,
  pub limit: Option<usize>,
}
