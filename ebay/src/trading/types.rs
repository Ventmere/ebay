#[derive(Debug, Default, Serialize, FromXmlElement)]
pub struct PaginationResult {
  pub total_number_of_pages: i64,
  pub total_number_of_entries: i64,
}

#[derive(Debug, Default, Serialize, FromXmlElement)]
pub struct ResponseMeta {
  pub timestamp: String,
  pub ack: String,
  pub version: String,
  pub build: String,
}
