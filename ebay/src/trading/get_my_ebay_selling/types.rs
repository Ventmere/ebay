use crate::trading::types::PaginationResult;

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct Response {
  pub active_list: ItemList,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct ItemList {
  #[from = "child_element"]
  pub item_array: Vec<Item>,
  pub pagination_result: PaginationResult,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct Item {
  pub buy_it_now_price: Price,
  pub item_id: String,
  pub listing_details: ListingDetails,
  pub listing_duration: String,
  pub listing_type: String,
  pub quantity: i64,
  pub selling_status: SellingStatus,
  pub shipping_details: ShippingDetails,
  pub time_left: String,
  pub title: String,
  pub watch_count: i64,
  pub question_count: i64,
  pub quantity_available: i64,
  pub sku: String,
  pub picture_details: PictureDetails,
  pub new_lead_count: String,
  pub classified_ad_pay_per_lead_fee: Price,
  pub seller_profiles: SellerProfiles,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct Price {
  #[from = "attr"]
  pub currency_id: String,
  #[from = "text"]
  pub amount: String,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct ListingDetails {
  pub start_time: String,
  pub view_item_url: String,
  pub view_item_url_for_natural_search: String,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct PictureDetails {
  pub gallery_url: String,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct SellerProfiles {
  pub seller_shipping_profile: SellerShippingProfile,
  pub seller_return_profile: SellerReturnProfile,
  pub seller_payment_profile: SellerPaymentProfile,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct SellerPaymentProfile {
  pub payment_profile_id: String,
  pub payment_profile_name: String,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct SellerReturnProfile {
  pub return_profile_id: String,
  pub return_profile_name: String,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct SellerShippingProfile {
  pub shipping_profile_id: String,
  pub shipping_profile_name: String,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct SellingStatus {
  pub current_price: Price,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct ShippingDetails {
  pub shipping_service_options: ShippingServiceOptions,
  pub shipping_type: String,
}

#[derive(Debug, Serialize, FromXmlElement, Default)]
pub struct ShippingServiceOptions {
  pub shipping_service_cost: Price,
}
