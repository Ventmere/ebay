mod types;

pub use self::types::*;

#[test]
fn test_item_from_xml_element() {
  use std::io::Cursor;
  use xmltree::Element;
  let xml = r##"
    <Item>
      <BuyItNowPrice currencyID="USD">129.99</BuyItNowPrice>
      <ItemID>123456789012</ItemID>
      <ListingDetails>
          <StartTime>2017-12-22T16:04:02.000Z</StartTime>
          <ViewItemURL>https://ViewItemURL</ViewItemURL>
          <ViewItemURLForNaturalSearch>http://ViewItemURLForNaturalSearch</ViewItemURLForNaturalSearch>
      </ListingDetails>
      <ListingDuration>GTC</ListingDuration>
      <ListingType>StoresFixedPrice</ListingType>
      <Quantity>139</Quantity>
      <SellingStatus>
          <CurrentPrice currencyID="USD">129.99</CurrentPrice>
      </SellingStatus>
      <ShippingDetails>
          <ShippingServiceOptions>
              <ShippingServiceCost currencyID="USD">0.0</ShippingServiceCost>
          </ShippingServiceOptions>
          <ShippingType>Flat</ShippingType>
      </ShippingDetails>
      <TimeLeft>PT11H12M15S</TimeLeft>
      <Title>Product Title</Title>
      <WatchCount>126</WatchCount>
      <QuestionCount>8</QuestionCount>
      <QuantityAvailable>49</QuantityAvailable>
      <SKU>product-sku</SKU>
      <PictureDetails>
          <GalleryURL>http://GalleryURL</GalleryURL>
      </PictureDetails>
      <NewLeadCount>8</NewLeadCount>
      <ClassifiedAdPayPerLeadFee currencyID="USD">0.0</ClassifiedAdPayPerLeadFee>
      <SellerProfiles>
          <SellerShippingProfile>
              <ShippingProfileID>123456789012</ShippingProfileID>
              <ShippingProfileName>One Day Handling</ShippingProfileName>
          </SellerShippingProfile>
          <SellerReturnProfile>
              <ReturnProfileID>123456789012</ReturnProfileID>
              <ReturnProfileName>Returns Accepted, Seller, 30 Days, Money back or exchange</ReturnProfileName>
          </SellerReturnProfile>
          <SellerPaymentProfile>
              <PaymentProfileID>123456789012</PaymentProfileID>
              <PaymentProfileName>PayPal:Immediate pay</PaymentProfileName>
          </SellerPaymentProfile>
      </SellerProfiles>
  </Item>
  "##.trim();
  let elem = Element::parse(Cursor::new(xml.as_bytes())).unwrap();
  let item = Item::from_xml_element(elem).unwrap();
  assert_eq!(
    format!("{:#?}", item),
    r##"Item {
    buy_it_now_price: Price {
        currency_id: "USD",
        amount: "129.99"
    },
    item_id: "123456789012",
    listing_details: ListingDetails {
        start_time: "2017-12-22T16:04:02.000Z",
        view_item_url: "https://ViewItemURL",
        view_item_url_for_natural_search: "http://ViewItemURLForNaturalSearch"
    },
    listing_duration: "GTC",
    listing_type: "StoresFixedPrice",
    quantity: 139,
    selling_status: SellingStatus {
        current_price: Price {
            currency_id: "USD",
            amount: "129.99"
        }
    },
    shipping_details: ShippingDetails {
        shipping_service_options: ShippingServiceOptions {
            shipping_service_cost: Price {
                currency_id: "USD",
                amount: "0.0"
            }
        },
        shipping_type: "Flat"
    },
    time_left: "PT11H12M15S",
    title: "Product Title",
    watch_count: 126,
    question_count: 8,
    quantity_available: 49,
    sku: "product-sku",
    picture_details: PictureDetails {
        gallery_url: "http://GalleryURL"
    },
    new_lead_count: "8",
    classified_ad_pay_per_lead_fee: Price {
        currency_id: "USD",
        amount: "0.0"
    },
    seller_profiles: SellerProfiles {
        seller_shipping_profile: SellerShippingProfile {
            shipping_profile_id: "123456789012",
            shipping_profile_name: "One Day Handling"
        },
        seller_return_profile: SellerReturnProfile {
            return_profile_id: "123456789012",
            return_profile_name: "Returns Accepted, Seller, 30 Days, Money back or exchange"
        },
        seller_payment_profile: SellerPaymentProfile {
            payment_profile_id: "123456789012",
            payment_profile_name: "PayPal:Immediate pay"
        }
    }
}"##
  )
}
