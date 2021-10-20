use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct MigrateListing {
  #[serde(rename = "listingId")]
  pub listing_id: String,
}

#[derive(Debug, Serialize)]
pub struct BulkMigrateListing {
  pub requests: Vec<MigrateListing>,
}

impl BulkMigrateListing {
  pub fn from_ids(ids: &Vec<&str>) -> BulkMigrateListing {
    BulkMigrateListing {
      requests: ids
        .into_iter()
        .map(|v| MigrateListing {
          listing_id: v.to_string(),
        }).collect(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkMigrateListingResponse {
  pub responses: Vec<MigrateListingResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrateListingResponse {
  #[serde(rename = "statusCode")]
  pub status_code: i64,
  #[serde(rename = "listingId")]
  pub listing_id: Option<String>,
  #[serde(rename = "inventoryItemGroupKey")]
  pub inventory_item_group_key: Option<String>,
  #[serde(rename = "marketplaceId")]
  pub marketplace_id: String,
  #[serde(rename = "inventoryItems")]
  pub inventory_items: Option<Vec<InventoryItemListing>>,
  pub errors: Option<Vec<ErrorDetailV3>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetailV3 {
  #[serde(rename = "errorId")]
  pub error_id: i64,
  pub domain: String,
  pub subdomain: String,
  pub category: String,
  pub message: String,
  pub parameters: Vec<ErrorParameterV3>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorParameterV3 {
  pub value: String,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItemListing {
  pub sku: String,
  #[serde(rename = "offerId")]
  pub offer_id: String,
}
