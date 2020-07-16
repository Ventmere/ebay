use super::types::ResponseMeta;
use reqwest::Response;
use result::*;
pub use xmltree::Element;

pub trait FromXmlElement: Sized + Default {
  fn from_xml_element(elem: &Element) -> EbayResult<Self>;
}

impl FromXmlElement for Option<Element> {
  fn from_xml_element(elem: &Element) -> EbayResult<Self> {
    Ok(Some(elem.clone()))
  }
}

impl FromXmlElement for i64 {
  fn from_xml_element(elem: &Element) -> EbayResult<Self> {
    let v = match elem.text {
      Some(ref text) => text
        .parse()
        .map_err(|err| format!("parse error: {:?}", err))?,
      None => Default::default(),
    };
    Ok(v)
  }
}

impl FromXmlElement for String {
  fn from_xml_element(elem: &Element) -> EbayResult<Self> {
    let v = match elem.text {
      Some(ref text) => text.clone(),
      None => Default::default(),
    };
    Ok(v)
  }
}

impl FromXmlElement for () {
  fn from_xml_element(_elem: &Element) -> EbayResult<()> {
    Ok(())
  }
}

impl<T> FromXmlElement for Vec<T>
where
  T: FromXmlElement,
{
  fn from_xml_element(elem: &Element) -> EbayResult<Self> {
    use std::iter::FromIterator;
    FromIterator::from_iter(elem.children.iter().map(T::from_xml_element))
  }
}

pub struct XmlResponse<T> {
  inner: T,
  meta: ResponseMeta,
  elem: Element,
  text: String,
}

impl<T> ::std::ops::Deref for XmlResponse<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> XmlResponse<T>
where
  T: FromXmlElement,
{
  pub fn parse(res: &mut Response) -> EbayResult<Self> {
    let text = res.text()?;
    Self::parse_string(text)
  }

  pub fn parse_string(text: String) -> EbayResult<Self> {
    use std::io::Cursor;

    let elem =
      Element::parse(Cursor::new(text.as_bytes())).map_err(|err| EbayError::Deserialize {
        msg: format!("parse response xml: {}", err.to_string()),
        body: text.to_string(),
      })?;

    let meta = ResponseMeta::from_xml_element(&elem).map_err(|err| EbayError::Deserialize {
      msg: format!("parse response meta error: {}", err),
      body: text.to_string(),
    })?;

    if meta.ack == "Failure" {
      use super::types::Error;
      use std::iter::FromIterator;
      let errors: Vec<Error> = EbayResult::<_>::from_iter(
        elem
          .children
          .iter()
          .filter(|elem| elem.name == "Errors")
          .map(Error::from_xml_element),
      )
      .map_err(|err| EbayError::Deserialize {
        msg: format!("parse errors error: {}", err),
        body: text.to_string(),
      })?;
      return Err(EbayError::TradingApiResponseError(errors));
    }

    let inner = T::from_xml_element(&elem)?;

    Ok(XmlResponse {
      inner,
      text,
      elem,
      meta,
    })
  }

  pub fn meta(&self) -> &ResponseMeta {
    &self.meta
  }

  pub fn text(&self) -> &str {
    self.text.as_ref()
  }

  pub fn element(&self) -> &Element {
    &self.elem
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
macro_rules! ebay_xml_element {
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
        ebay_xml_element!($tag_name $attrs $children)
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
    use $crate::trading::Element;
    let mut elem = Element::new(stringify!($tag_name));
    ebay_xml_element!(ATTRS elem, $attrs);
    ebay_xml_element!(CHILDREN elem, $children);
    elem
  }};
}

#[test]
fn test_xml_element() {
  use std::io::Cursor;
  let a = 1;
  let elem = ebay_xml_element!(
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
  assert_eq!(
    xml_text,
    r##"<?xml version="1.0" encoding="UTF-8"?><GetMyeBaySellingRequest xmlns="urn:ebay:apis:eBLBaseComponents"><RequesterCredentials a="1"><eBayAuthToken>AUTH_TOKEN</eBayAuthToken></RequesterCredentials></GetMyeBaySellingRequest>"##
  );
}
