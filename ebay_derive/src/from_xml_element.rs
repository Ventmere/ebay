use crate::proc_macro::TokenStream;
use std::collections::BTreeMap;
use syn::spanned::Spanned;
use syn::*;

enum FieldFrom {
  ChildElement,
  ChildElements,
  Attr,
  Text,
}

impl FieldFrom {
  fn parse(v: &str) -> Self {
    match v {
      "text" => FieldFrom::Text,
      "attr" => FieldFrom::Attr,
      "child_elements" => FieldFrom::ChildElements,
      "child_element" => FieldFrom::ChildElement,
      _ => panic!("invalid `from` attr value '{}'", v),
    }
  }
}

pub fn derive(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse(input).unwrap();
  let name = &ast.ident;

  if ast.generics.lifetimes().count() > 0 {
    panic!("generics not supported")
  }

  let fields: Vec<&Field> = match ast.data {
    Data::Struct(DataStruct {
      fields: Fields::Named(FieldsNamed { ref named, .. }),
      ..
    }) => named.iter().collect(),
    _ => panic!("only supports non-unit struct with named fields"),
  };

  let lines: Vec<_> = fields
    .into_iter()
    .map(|field| {
      let name = field.ident.as_ref().unwrap();
      let tag_name = field_name_to_tag_name(&name.to_string());
      let attrs: BTreeMap<String, String> = field
        .attrs
        .iter()
        .filter_map(|a| {
          a.interpret_meta().and_then(|meta| {
            if let Meta::NameValue(MetaNameValue {
              ref ident,
              lit: Lit::Str(ref lit),
              ..
            }) = meta
            {
              Some((ident.to_string(), lit.value()))
            } else {
              None
            }
          })
        })
        .collect();

      let from = attrs
        .get("from")
        .map(|v| FieldFrom::parse(v))
        .unwrap_or_else(|| FieldFrom::ChildElement);
      match from {
        FieldFrom::ChildElement => {
          quote_spanned! {name.span() =>
            #name: match elem.get_child(#tag_name) {
              Some(elem) => {
                crate::trading::xml_helper::FromXmlElement::from_xml_element(elem)?
              },
              None => Default::default(),
            }
          }
        }
        FieldFrom::ChildElements => {
          quote_spanned! {name.span() =>
            #name: crate::trading::xml_helper::FromXmlElement::from_xml_element(&elem)?
          }
        }
        FieldFrom::Attr => {
          let attr_name = field_name_to_attr_name(&name.to_string());
          quote_spanned! {field.span() =>
            #name: elem.attributes.get(#attr_name).cloned().unwrap_or_default()
          }
        }
        FieldFrom::Text => {
          quote_spanned! {field.span() =>
            #name: elem.text.clone().unwrap_or_default().parse().map_err(|err| {
              format!("parse field error: {:?}", err)
            })?,
          }
        }
      }
    })
    .collect();

  let output: proc_macro2::TokenStream = quote!{
    impl crate::trading::xml_helper::FromXmlElement for #name {
      fn from_xml_element(mut elem: &::xmltree::Element) -> crate::result::EbayResult<Self> {
        Ok(
          #name {
            #(#lines),*
          }
        )
      }
    }
  };

  output.into()
}

const ABBR_WORDS: &'static [&'static str] = &["id", "url", "sku"];

/// `classified_ad_pay_per_lead_fee` => `ClassifiedAdPayPerLeadFee`
/// `item_id` => `ItemID`
fn field_name_to_tag_name(field_name: &str) -> String {
  let words: Vec<String> = field_name
    .split('_')
    .filter_map(|word| {
      let word = word.trim();
      if word.is_empty() {
        return None;
      }

      let transformed = if ABBR_WORDS.contains(&word) {
        word.to_uppercase()
      } else {
        let mut word = word.to_string();
        word.replace_range(0..1, &word.get(0..1).unwrap().to_uppercase().to_string());
        word
      };
      Some(transformed)
    })
    .collect();

  words.join("")
}

#[test]
fn test_field_name_to_tag_name() {
  let cases = [
    ["item_id", "ItemID"],
    [
      "classified_ad_pay_per_lead_fee",
      "ClassifiedAdPayPerLeadFee",
    ],
  ];

  for pair in cases.into_iter() {
    assert_eq!(field_name_to_tag_name(pair[0]), pair[1])
  }
}

/// `currency_id` => `currencyID`
fn field_name_to_attr_name(field_name: &str) -> String {
  let words: Vec<String> = field_name
    .split('_')
    .enumerate()
    .filter_map(|(i, word)| {
      if i == 0 {
        return Some(word.to_string());
      }
      let word = word.trim();
      if word.is_empty() {
        return None;
      }

      let transformed = if ABBR_WORDS.contains(&word) {
        word.to_uppercase()
      } else {
        let mut word = word.to_string();
        word.replace_range(0..1, &word.get(0..1).unwrap().to_uppercase().to_string());
        word
      };
      Some(transformed)
    })
    .collect();

  words.join("")
}

#[test]
fn test_field_name_to_attr_name() {
  let cases = [["currency_id", "currencyID"]];

  for pair in cases.into_iter() {
    assert_eq!(field_name_to_attr_name(pair[0]), pair[1])
  }
}

// fn is_vec(ty: &Type) -> bool {
//   if let Type::Path(TypePath {
//     path: Path { ref segments, .. },
//     ..
//   }) = *ty
//   {
//     if let Some(ident) = segments.first().map(|pair| pair.value().ident.clone()) {
//       ident == "Vec"
//     } else {
//       false
//     }
//   } else {
//     false
//   }
// }
