#[macro_export]
macro_rules! xml_element {
  (
    ATTRS $e:expr, [
      $($attr_name:ident = $attr_value:expr),*
    ]
  ) => {
    $(
      $e.attributes.insert(stringify!($attr_name).to_string(), $attr_value.to_string());
    )*
  };

  (
    CHILDREN $e:expr, [
      $($tag_name:ident $attrs:tt $children:tt)*
    ]
  ) => {
    $(
      $e.children.push(
        xml_element!($tag_name $attrs $children)
      );
    )*
  };

  (
    CHILDREN $e:expr, [
      $text:expr
    ]
  ) => {
    $e.text = Some($text.to_string());
  };

  (
    $tag_name:ident $attrs:tt $children:tt
  ) => {{
    use xmltree::Element;
    let mut elem = Element::new(stringify!($tag_name));
    xml_element!(ATTRS elem, $attrs);
    xml_element!(CHILDREN elem, $children);
    elem
  }};
}

#[test]
fn test_xml_element() {
  use std::io::Cursor;
  let a = 1;
  let elem = xml_element!(
    GetMyeBaySellingRequest[xmlns="urn:ebay:apis:eBLBaseComponents"][
      RequesterCredentials[a=a][
        eBayAuthToken[][
          "AUTH_TOKEN"
        ]
      ]
    ]
  );
  let mut buf: Cursor<Vec<u8>> = Cursor::new(vec![]);
  elem.write(&mut buf).unwrap();
  let xml_text = String::from_utf8(buf.into_inner()).unwrap();
  assert_eq!(xml_text, r##"<?xml version="1.0" encoding="UTF-8"?><GetMyeBaySellingRequest xmlns="urn:ebay:apis:eBLBaseComponents"><RequesterCredentials a="1"><eBayAuthToken>AUTH_TOKEN</eBayAuthToken></RequesterCredentials></GetMyeBaySellingRequest>"##);
}
