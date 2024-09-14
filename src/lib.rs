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
///
/// ```rust
/// #[e_macros::value]
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// enum ApiStatus {
///     #[e(value = "OK", index = 200)]
///     Ok,
///     #[e(value = "NOT_FOUND", index = 404)]
///     NotFound(String),
///     #[e(value = "SERVER_ERROR", index = 500)]
///     ServerError { message: String },
/// }
///
/// // Usage:
/// let status = ApiStatus::NotFound("Resource not available".to_string());
///
/// // Debug output
/// println!("{:?}", status);
/// // Output: NotFound("Resource not available")
///
/// // Display output
/// println!("{}", status);
/// // Output: NOT_FOUND
///
/// // Serde serialization
/// let json = status.to_serde().unwrap();
/// println!("Serialized: {}", json);
/// // Output: Serialized: {"NotFound":"Resource not available"}
///
/// // Serde deserialization
/// let deserialized: ApiStatus = ApiStatus::from_serde(serde_json::json!({
///     "ServerError": { "message": "Internal server error" }
/// })).unwrap();
/// assert!(matches!(deserialized, ApiStatus::ServerError { .. }));
/// ```
///
/// # Example: Configuration Enum with Debug
///
/// ```rust
/// #[e_macros::value]
/// #[derive(Debug, PartialEq)]
/// enum Config {
///     #[e(value = "database")]
///     Database { url: String, port: u16 },
///     #[e(value = "api")]
///     Api { endpoint: String },
///     #[e(value = "logging")]
///     Logging(LogLevel),
/// }
///
/// #[e_macros::value]
/// #[derive(Debug, PartialEq)]
/// enum LogLevel {
///     #[e(value = "debug")]
///     Debug,
///     #[e(value = "info")]
///     Info,
/// }
///
/// let config = Config::Database {
///     url: "localhost".to_string(),
///     port: 5432,
/// };
///
/// // Detailed debug output
/// println!("{:?}", config);
/// // Output: Database { url: "localhost", port: 5432 }
///
/// // Simple display output
/// println!("{}", config);
/// // Output: database
/// ```
///
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
