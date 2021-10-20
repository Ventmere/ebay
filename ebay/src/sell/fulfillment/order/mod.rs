use chrono::{DateTime, SecondsFormat, Utc};
use serde::ser::{Serializer};
use serde::{Serialize};

mod types;

pub use self::types::*;
pub mod shipping_fulfillment;

#[derive(Debug, Serialize, Default)]
pub struct GetOrdersParams {
  pub offset: Option<i32>,
  pub limit: Option<usize>,
  pub filter: Option<Filter>,
  #[serde(skip_serializing)]
  pub order_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy)]
enum DateRange {
  After(DateTime<Utc>),
  Between(DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Default)]
pub struct Filter {
  creation_date: Option<DateRange>,
  last_modified_date: Option<DateRange>,
  order_fulfillment_status: Option<&'static str>,
}

impl Filter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn created_after(self, date: DateTime<Utc>) -> Self {
    Self {
      creation_date: Some(DateRange::After(date)),
      last_modified_date: self.last_modified_date,
      order_fulfillment_status: self.order_fulfillment_status,
    }
  }

  pub fn created_between(self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
    Self {
      creation_date: Some(DateRange::Between(from, to)),
      last_modified_date: self.last_modified_date,
      order_fulfillment_status: self.order_fulfillment_status,
    }
  }

  pub fn modified_after(self, date: DateTime<Utc>) -> Self {
    Self {
      creation_date: self.creation_date,
      last_modified_date: Some(DateRange::After(date)),
      order_fulfillment_status: self.order_fulfillment_status,
    }
  }

  pub fn modified_between(self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
    Self {
      creation_date: self.creation_date,
      last_modified_date: Some(DateRange::Between(from, to)),
      order_fulfillment_status: self.order_fulfillment_status,
    }
  }

  /// orderfulfillmentstatus:{NOT_STARTED|IN_PROGRESS}
  /// specifies orders for which no shipping fulfillments have been
  /// started, plus orders for which at least one shipping fulfillment
  /// has been started but not completed.
  pub fn status_not_started_in_progress(self) -> Self {
    Self {
      creation_date: self.creation_date,
      last_modified_date: self.last_modified_date,
      order_fulfillment_status: Some("{NOT_STARTED|IN_PROGRESS}"),
    }
  }

  /// orderfulfillmentstatus:{FULFILLED|IN_PROGRESS}
  /// specifies orders for which all shipping fulfillments have been
  /// completed, plus orders for which at least one shipping fulfillment
  /// has been started but not completed.
  pub fn status_fulfilled_in_progress(self) -> Self {
    Self {
      creation_date: self.creation_date,
      last_modified_date: self.last_modified_date,
      order_fulfillment_status: Some("{FULFILLED|IN_PROGRESS}"),
    }
  }
}

fn format_date(date: &DateTime<Utc>) -> String {
  date.to_rfc3339_opts(SecondsFormat::Millis, true)
}

impl Serialize for Filter {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut encoded = String::new();
    match self.creation_date.as_ref() {
      Some(DateRange::After(date)) => {
        encoded.push_str(&format!("creationdate:[{}..]", format_date(date)))
      }
      Some(DateRange::Between(from, to)) => encoded.push_str(&format!(
        "creationdate:[{}..{}]",
        format_date(from),
        format_date(to)
      )),
      None => {}
    }

    match self.last_modified_date.as_ref() {
      Some(range) => {
        if !encoded.is_empty() {
          encoded.push(',')
        }
        match range {
          DateRange::After(date) => {
            encoded.push_str(&format!("lastmodifieddate:[{}..]", format_date(date)))
          }
          DateRange::Between(from, to) => encoded.push_str(&format!(
            "lastmodifieddate:[{}..{}]",
            format_date(from),
            format_date(to)
          )),
        }
      }
      None => {}
    }

    if let Some(v) = self.order_fulfillment_status {
      if !encoded.is_empty() {
        encoded.push(',')
      }

      encoded.push_str("orderfulfillmentstatus:");
      encoded.push_str(v)
    }

    encoded.serialize::<S>(serializer)
  }
}
