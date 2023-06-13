use chrono::{DateTime, Utc};
use serde_json::Value;
use serde::{Serialize, Deserialize};

uppercase_str_enum! {
  pub enum OrderFulfillmentStatus {
    Fulfilled,
    InProgress,
    NotStarted,
  }
}

uppercase_str_enum! {
  pub enum OrderPaymentStatus {
    Failed,
    FullyRefunded,
    Paid,
    PartiallyRefunded,
    Pending,
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  #[serde(rename = "orderId")]
  pub order_id: String,
  #[serde(rename = "creationDate")]
  pub creation_date: DateTime<Utc>,
  #[serde(rename = "lastModifiedDate")]
  pub last_modified_date: DateTime<Utc>,
  #[serde(rename = "orderFulfillmentStatus")]
  pub order_fulfillment_status: OrderFulfillmentStatus,
  #[serde(rename = "orderPaymentStatus")]
  pub order_payment_status: OrderPaymentStatus,
  #[serde(rename = "sellerId")]
  pub seller_id: String,
  pub buyer: Buyer,
  #[serde(rename = "buyerCheckoutNotes")]
  pub buyer_checkout_notes: Option<String>,
  #[serde(rename = "pricingSummary")]
  pub pricing_summary: PricingSummary,
  #[serde(rename = "paymentSummary")]
  pub payment_summary: PaymentSummary,
  #[serde(rename = "fulfillmentStartInstructions")]
  pub fulfillment_start_instructions: Vec<FulfillmentStartInstruction>,
  #[serde(rename = "fulfillmentHrefs")]
  pub fulfillment_hrefs: Option<Vec<String>>,
  #[serde(rename = "lineItems")]
  pub line_items: Vec<LineItem>,
  #[serde(rename = "cancelStatus")]
  pub cancel_status: CancelStatus,
}

uppercase_str_enum! {
  pub enum CancelState {
    Canceled,
    InProgress,
    NoneRequested,
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelStatus {
  #[serde(rename = "cancelRequests")]
  pub cancel_requests: Vec<CancelRequest>,
  #[serde(rename = "cancelState")]
  pub cancel_state: CancelState,
  #[serde(rename = "cancelledDate")]
  pub cancelled_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelRequest {
  #[serde(rename = "cancelCompletedDate")]
  pub cancel_completed_date: Option<String>,
  #[serde(rename = "cancelInitiator")]
  pub cancel_initiator: Option<String>,
  #[serde(rename = "cancelRequestId")]
  pub cancel_request_id: Option<String>,
  #[serde(rename = "cancelRequestState")]
  pub cancel_request_state: Option<String>,
  #[serde(rename = "cancelRequestedDate")]
  pub cancel_requested_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSummary {
  pub payments: Vec<Payment>,
  pub refunds: Vec<OrderRefund>,
  #[serde(rename = "totalDueSeller")]
  pub total_due_seller: Option<Amount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRefund {
  pub amount: Amount,
  #[serde(rename = "refundDate")]
  pub refund_date: Option<DateTime<Utc>>,
  #[serde(rename = "refundReferenceId")]
  pub refund_reference_id: String,
  #[serde(rename = "refundStatus")]
  pub refund_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
  pub amount: Amount,
  #[serde(rename = "paymentDate")]
  pub payment_date: Option<DateTime<Utc>>,
  #[serde(rename = "paymentMethod")]
  pub payment_method: String,
  #[serde(rename = "paymentReferenceId")]
  pub payment_reference_id: String,
  #[serde(rename = "paymentStatus")]
  pub payment_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingSummary {
  pub adjustment: Option<Amount>,
  #[serde(rename = "deliveryDiscount")]
  pub delivery_discount: Option<Amount>,
  #[serde(rename = "deliveryCost")]
  pub delivery_cost: Option<Amount>,
  #[serde(rename = "priceSubtotal")]
  pub price_subtotal: Amount,
  pub total: Amount,
  pub fee: Option<Amount>,
  pub tax: Option<Amount>,
  #[serde(rename = "priceDiscountSubtotal")]
  pub price_discount_subtotal: Option<Amount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buyer {
  pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentStartInstruction {
  #[serde(default)]
  #[serde(rename = "ebaySupportedFulfillment")]
  pub ebay_supported_fulfillment: bool,
  #[serde(rename = "fulfillmentInstructionsType")]
  pub fulfillment_instructions_type: String,
  #[serde(rename = "maxEstimatedDeliveryDate")]
  pub max_estimated_delivery_date: Option<DateTime<Utc>>,
  #[serde(rename = "minEstimatedDeliveryDate")]
  pub min_estimated_delivery_date: Option<DateTime<Utc>>,
  #[serde(rename = "shippingStep")]
  pub shipping_step: Option<ShippingStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingStep {
  #[serde(rename = "shipTo")]
  pub ship_to: Contact,
  #[serde(rename = "shippingServiceCode")]
  pub shipping_service_code: String,
  #[serde(rename = "shipToReferenceId")]
  pub ship_to_reference_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
  #[serde(rename = "contactAddress")]
  pub contact_address: Address,
  #[serde(rename = "fullName")]
  #[serde(default)]
  pub full_name: String,
  #[serde(rename = "primaryPhone")]
  #[serde(default)]
  pub primary_phone: PhoneNumber,
  #[serde(default)]
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  #[serde(rename = "addressLine1")]
  pub address_line1: Option<String>,
  #[serde(rename = "addressLine2")]
  pub address_line2: Option<String>,
  pub city: String,
  #[serde(rename = "countryCode")]
  pub country_code: String,
  #[serde(rename = "postalCode")]
  #[serde(default)]
  pub postal_code: String,
  #[serde(rename = "stateOrProvince")]
  pub state_or_province: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PhoneNumber {
  #[serde(rename = "phoneNumber")]
  pub phone_number: Option<String>,
}

uppercase_str_enum! {
  pub enum LineItemFulfillmentStatus {
    Fulfilled,
    NotStarted,
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tax {
  pub amount: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
  #[serde(rename = "appliedPromotions")]
  pub applied_promotions: Vec<AppliedPromotion>,
  #[serde(rename = "deliveryCost")]
  pub delivery_cost: DeliveryCost,
  #[serde(rename = "legacyItemId")]
  pub legacy_item_id: String,
  /// The selling price of the line item before applying
  /// any discounts. The value of this field is calculated
  /// by multiplying the single unit price by the value of
  /// the quantity field.
  #[serde(rename = "lineItemCost")]
  pub line_item_cost: Amount,
  #[serde(rename = "lineItemFulfillmentInstructions")]
  pub line_item_fulfillment_instructions: LineItemFulfillmentInstructions,
  #[serde(rename = "lineItemFulfillmentStatus")]
  pub line_item_fulfillment_status: LineItemFulfillmentStatus,
  #[serde(rename = "lineItemId")]
  pub line_item_id: String,
  #[serde(rename = "listingMarketplaceId")]
  pub listing_marketplace_id: String,
  pub properties: Option<LineItemProperties>,
  #[serde(rename = "purchaseMarketplaceId")]
  pub purchase_marketplace_id: String,
  pub quantity: i32,
  pub sku: String,
  #[serde(rename = "soldFormat")]
  pub sold_format: String,
  pub taxes: Vec<Tax>,
  pub title: String,
  pub total: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppliedPromotion {
  pub description: String,
  #[serde(rename = "discountAmount")]
  pub discount_amount: Amount,
  #[serde(rename = "promotionId")]
  pub promotion_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryCost {
  #[serde(rename = "shippingCost")]
  pub shipping_cost: Amount,
  #[serde(rename = "importCharges")]
  pub import_charges: Option<Amount>,
  #[serde(rename = "shippingIntermediationFee")]
  pub shipping_intermediation_fee: Option<Amount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
  #[serde(rename = "convertedFromCurrency")]
  pub converted_from_currency: Option<String>,
  #[serde(rename = "convertedFromValue")]
  pub converted_from_value: Option<String>,
  pub currency: String,
  pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItemFulfillmentInstructions {
  #[serde(rename = "guaranteedDelivery")]
  pub guaranteed_delivery: bool,
  #[serde(rename = "maxEstimatedDeliveryDate")]
  pub max_estimated_delivery_date: Option<DateTime<Utc>>,
  #[serde(rename = "minEstimatedDeliveryDate")]
  pub min_estimated_delivery_date: Option<DateTime<Utc>>,
  #[serde(rename = "shipByDate")]
  pub ship_by_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LineItemProperties {
  #[serde(default)]
  #[serde(rename = "buyerProtection")]
  pub buyer_protection: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSearchPagedCollection {
  pub href: String,
  pub limit: Option<i32>,
  pub next: Option<String>,
  pub offset: Option<i32>,
  pub orders: Vec<Order>,
  pub prev: Option<String>,
  pub total: i32,
  pub warnings: Option<Vec<Value>>,
}
