use reqwest::Response;
use result::*;
use std::str::FromStr;
use xmltree::Element;

pub trait FromXmlElement: Sized + Default {
  fn from_xml_element(elem: Element) -> EbayResult<Self>;
}

impl<T> FromXmlElement for T
where
  T: FromStr + Default,
  T::Err: ::std::fmt::Debug,
{
  fn from_xml_element(elem: Element) -> EbayResult<Self> {
    let v = match elem.text {
      Some(text) => text
        .parse()
        .map_err(|err| format!("parse error: {:?}", err))?,
      None => Default::default(),
    };
    Ok(v)
  }
}

pub struct Xml<T> {
  inner: T,
  text: String,
}

impl<T> ::std::ops::Deref for Xml<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> Xml<T>
where
  T: FromXmlElement,
{
  pub fn from_res(res: &mut Response) -> EbayResult<Self> {
    use std::io::Cursor;

    let text = res.text()?;
    let elem =
      Element::parse(Cursor::new(text.as_bytes())).map_err(|err| EbayError::Deserialize {
        msg: format!("parse response xml: {}", err.to_string()),
        body: text.clone(),
      })?;

    let inner = T::from_xml_element(elem)?;

    Ok(Xml { inner, text })
  }

  pub fn text(&self) -> &str {
    self.text.as_ref()
  }

  pub fn into_inner(self) -> T {
    self.inner
  }
}

pub trait GetChildText {
  fn get_child_text(&self, name: &str) -> Option<String>;
  fn get_child_text_or_default(&self, name: &str) -> String {
    self.get_child_text(name).unwrap_or_default()
  }
}

impl GetChildText for Element {
  fn get_child_text(&self, name: &str) -> Option<String> {
    self.get_child(name).and_then(|c| c.text.clone())
  }
}

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
