#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

extern crate syn;
#[macro_use]
extern crate quote;

use crate::proc_macro::TokenStream;

mod from_xml_element;

#[proc_macro_derive(FromXmlElement, attributes(from, tag_name, attr_name))]
pub fn derive_from_xml_element(input: TokenStream) -> TokenStream {
  from_xml_element::derive(input)
}
