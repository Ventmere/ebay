//! Fulfillment API
//! [Doc](https://developer.ebay.com/api-docs/sell/inventory/overview.html)

use client::*;
use result::EbayResult;

pub mod inventory_item;
pub mod listing;
pub mod location;
pub mod offer;

use self::inventory_item::{GetInventoryItemsParams, InventoryItems};
use self::listing::{BulkMigrateListing, BulkMigrateListingResponse};
use self::location::{GetLocationParams, LocationResponse};
use self::offer::{GetOfferParams, Offers};

pub trait InventoryApi {
  fn get_inventory_locations(&self, params: &GetLocationParams) -> EbayResult<LocationResponse>;
  fn get_inventory_items(&self, params: &GetInventoryItemsParams) -> EbayResult<InventoryItems>;
  fn get_offers(&self, params: &GetOfferParams) -> EbayResult<Offers>;
  fn bulk_migrate_listing(
    &self,
    params: &BulkMigrateListing,
  ) -> EbayResult<BulkMigrateListingResponse>;
}

impl InventoryApi for EbayClient {
  fn get_inventory_locations(&self, params: &GetLocationParams) -> EbayResult<LocationResponse> {
    self
      .request(Method::Get, "/sell/inventory/v1/location")?
      .query(params)
      .send()?
      .get_response()
  }

  fn get_inventory_items(&self, params: &GetInventoryItemsParams) -> EbayResult<InventoryItems> {
    self
      .request(Method::Get, "/sell/inventory/v1/inventory_item")?
      .query(params)
      .send()?
      .get_response()
  }

  fn get_offers(&self, params: &GetOfferParams) -> EbayResult<Offers> {
    self
      .request(Method::Get, "/sell/inventory/v1/offer")?
      .query(params)
      .send()?
      .get_response()
  }

  fn bulk_migrate_listing(
    &self,
    params: &BulkMigrateListing,
  ) -> EbayResult<BulkMigrateListingResponse> {
    self
      .request(Method::Post, "/sell/inventory/v1/bulk_migrate_listing")?
      .json(params)
      .send()?
      .get_response()
  }
}
