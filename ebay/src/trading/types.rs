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
  pub errors: Vec<Error>,
}

#[derive(Debug, Default, Serialize, FromXmlElement)]
pub struct Error {
  pub short_message: String,
  pub long_message: String,
  pub error_code: String,
  pub error_classification: String,
  pub severity_code: String,
  pub error_parameters: Vec<ErrorParameter>,
}
#[derive(Debug, Default, Serialize, FromXmlElement)]
pub struct ErrorParameter {
  #[from = "attr"]
  #[attr_name = "ParamID"]
  pub param_id: String,
  pub value: String,
}
