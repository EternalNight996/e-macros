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

/// Enhances enum types with additional functionality for serialization, deserialization, debugging, and more.
///
/// This macro generates the following for the target enum:
/// - Implementations of `Debug` and `Display` traits for easy logging and debugging
/// - Implementation of `TryFrom<&str>` for parsing from strings
/// - Custom value mappings for flexible serialization
/// - Numeric index support for efficient storage and retrieval
/// - `to_serde` and `from_serde` methods for JSON serialization (when `serde` feature is enabled)
///
/// # Features
///
/// - **Debug Output**: Provides detailed debug output, including variant names and associated data.
/// - **Display Implementation**: Customizable string representation for each variant.
/// - **Serde Integration**: Optional JSON serialization and deserialization support.
/// - **Numeric Indexing**: Allows efficient mapping between enum variants and numeric values.
///
/// # Example: API Status with Serde Support
/// ```rust
/// use e_macros::value;
/// #[value]
/// #[derive(Debug, PartialEq)]
/// enum Color {
///     #[e(value = "RED", index = 0)]
///     Red,
///     #[e(value = "GREEN", index = 1)]
///     Green,
///     #[e(value = "BLUE", index = 2)]
///     Blue,
/// }
/// fn main() {
///     let color = Color::Green;
///     println!("Color value: {}", color.value());
///     println!("Color index: {}", color.index());
///     let from_value = Color::try_from("BLUE").unwrap();
///     println!("From value: {:?}", from_value);
///     let from_index = Color::try_from(0).unwrap();
///     println!("From index: {:?}", from_index);
///     println!("Variant count: {}", Color::variant_count());
/// }
/// ```
/// # Notes
///
/// - String conversion is case-sensitive for precise matching
/// - Custom string values can be specified using `#[e(value = "...")]`
/// - Numeric indices can be assigned using `#[e(index = ...)]`
/// - The `to_serde` and `from_serde` methods are available when the `serde` feature is enabled
/// - Debug output includes full details of enum variants and their associated data
/// - Display output uses the custom `value` if specified, otherwise falls back to the variant name
///
/// This macro significantly reduces boilerplate code and enhances the functionality
/// of enums, making them more powerful and easier to use in various scenarios,
/// especially in applications requiring serialization, configuration management,
/// and detailed debugging.
#[proc_macro_attribute]
pub fn value(_: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input enum definition
    let enum_input = parse_macro_input!(item as syn::ItemEnum);
    
    // Generate additional structures and implementations
    _value::r#enum::create_structure(enum_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
