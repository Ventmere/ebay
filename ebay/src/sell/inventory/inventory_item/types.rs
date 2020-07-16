use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItems {
  pub href: Option<String>,
  pub limit: Option<String>,
  #[serde(rename = "inventoryItems")]
  pub inventory_items: Vec<InventoryItemWithSkuLocaleGroupid>,
  pub next: String,
  pub size: i64,
  pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItemWithSkuLocaleGroupid {
  pub sku: String,
  pub availability: Availability,
  pub condition: String,
  pub product: Product,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Availability {
  #[serde(rename = "shipToLocationAvailability")]
  pub ship_to_location_availability: ShipToLocationAvailability,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipToLocationAvailability {
  pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
  pub title: String,
  pub description: String,
  pub aspects: BTreeMap<String, Vec<String>>,
  #[serde(rename = "imageUrls")]
  pub image_urls: Vec<String>,
}
