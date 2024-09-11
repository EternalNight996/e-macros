#![allow(
    clippy::cognitive_complexity,
    clippy::large_enum_variant,
    clippy::module_inception,
    clippy::needless_doctest_main
)]
#![warn(
  missing_debug_implementations,
//   missing_docs,
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

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Enum, attributes(ename))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let variants = if let Data::Enum(data_enum) = &input.data {
        data_enum
            .variants
            .iter()
            .enumerate()
            .map(|(index, v)| {
                let ident = &v.ident;
                let mut description = ident.to_string();
                if let Some(attr) = v.attrs.iter().find(|a| a.path().is_ident("ename")) {
                    if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
                        description = lit.value();
                    }
                }
                (ident, index as i32, description)
            })
            .collect::<Vec<_>>()
    } else {
        panic!("Enum derive macro can only be used with enums");
    };

    let variant_idents: Vec<_> = variants.iter().map(|(ident, _, _)| ident).collect();
    let variant_indices: Vec<_> = variants.iter().map(|(_, index, _)| index).collect();
    let variant_name: Vec<_> = variants
        .iter()
        .map(|(ident, _, desc)| {
            if desc.is_empty() {
                quote! { stringify!(#ident) }
            } else {
                quote! { #desc }
            }
        })
        .collect();
    let variant_count = variants.len();
    let serde_impl = generate_serde_impl(name);
    let expanded = quote! {
        impl #name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #(Self::#variant_idents => #variant_name,)*
                }
            }

            pub const ALL: [Self; #variant_count] = [#(Self::#variant_idents),*];
        }

        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#variant_name | stringify!(#variant_idents) => Ok(Self::#variant_idents),)*
                    _ => Err(format!("Invalid string: {}", s)),
                }
            }
        }
        impl TryFrom<&str> for #name {
            type Error = String;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    #(#variant_name | stringify!(#variant_idents) => Ok(Self::#variant_idents),)*
                    _ => Err(format!("Invalid string: {}", s)),
                }
            }
        }

        impl TryFrom<i32> for #name {
            type Error = String;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    #(#variant_indices => Ok(Self::#variant_idents),)*
                    _ => Err(format!("Invalid integer: {}", value)),
                }
            }
        }

        impl From<#name> for i32 {
            fn from(value: #name) -> Self {
                value as i32
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }

        #serde_impl
    };

    TokenStream::from(expanded)
}

fn generate_serde_impl(name: &syn::Ident) -> TokenStream2 {
    quote! {
        #[cfg(feature = "serde")]
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse::<Self>().map_err(serde::de::Error::custom)
            }
        }
    }
}
