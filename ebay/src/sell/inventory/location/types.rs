#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  pub city: Option<String>,
  #[serde(rename = "stateOrProvince")]
  pub state_or_province: String,
  #[serde(rename = "postalCode")]
  pub postal_code: Option<String>,
  pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
  #[serde(rename = "locationId")]
  pub location_id: String,
  pub address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryLocationResponse {
  pub name: Option<String>,
  pub location: Location,
  #[serde(rename = "merchantLocationStatus")]
  pub merchant_location_status: String,
  #[serde(rename = "locationTypes")]
  pub location_types: Option<Vec<String>>,
  #[serde(rename = "merchantLocationKey")]
  pub merchant_location_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
  pub total: i64,
  pub locations: Vec<InventoryLocationResponse>,
}
