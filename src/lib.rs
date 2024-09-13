#![doc = include_str!("../README.md")]
#![allow(
    clippy::cognitive_complexity,
    clippy::large_enum_variant,
    clippy::module_inception,
    clippy::needless_doctest_main
)]
#![warn(
    missing_debug_implementations,
    rust_2021_compatibility,
    unreachable_pub
)]
#![deny(unused_must_use)]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2021_compatibility),
        allow(dead_code, unused_variables)
    )
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![cfg_attr(any(), allow(dead_code, unreachable_pub))]

extern crate proc_macro;
mod _value;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn value(_: TokenStream, item: TokenStream) -> TokenStream {
    let enum_input = parse_macro_input!(item as syn::ItemEnum);
    _value::r#enum::create_structure(enum_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
