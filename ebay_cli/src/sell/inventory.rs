use ebay::sell::inventory::*;
use helpers::get_client;
use serde_json;
use std::io::stdout;

pub fn bulk_migrate_listing(values: &Vec<&str>) {
  use ebay::sell::inventory::listing::*;
  let res = get_client()
    .bulk_migrate_listing(&BulkMigrateListing::from_ids(values))
    .unwrap();
  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}

pub fn get_inventory_locations() {
  use ebay::sell::inventory::location::*;
  let res = get_client()
    .get_inventory_locations(&GetLocationParams {
      limit: Some(100),
      ..Default::default()
    }).unwrap();
  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}

pub fn get_inventory_items() {
  use ebay::sell::inventory::inventory_item::*;
  let res = get_client()
    .get_inventory_items(&GetInventoryItemsParams {
      limit: Some(100),
      ..Default::default()
    }).unwrap();
  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}

pub fn get_offers(sku: &str) {
  use ebay::sell::inventory::offer::*;
  let res = get_client()
    .get_offers(&GetOfferParams {
      sku: sku.to_string(),
      limit: Some(100),
      ..Default::default()
    }).unwrap();
  serde_json::to_writer_pretty(stdout(), &res).unwrap();
}
