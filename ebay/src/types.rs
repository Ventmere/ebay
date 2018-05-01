use chrono::{DateTime, SecondsFormat, Utc};
use serde::ser::{Serialize, Serializer};

pub struct EbayDateTime(DateTime<Utc>);

impl EbayDateTime {
  pub fn new(v: DateTime<Utc>) -> Self {
    EbayDateTime(v)
  }
}

impl From<DateTime<Utc>> for EbayDateTime {
  fn from(v: DateTime<Utc>) -> Self {
    EbayDateTime(v)
  }
}

impl Serialize for EbayDateTime {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self
      .0
      .to_rfc3339_opts(SecondsFormat::Millis, true)
      .serialize::<S>(serializer)
  }
}
