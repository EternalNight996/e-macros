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
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
                (ident, index as i32, description, &v.fields)
            })
            .collect::<Vec<_>>()
    } else {
        panic!("Enum derive macro can only be used with enums");
    };
    let serde_impl = generate_serde_impl(name);
    let as_str_impl = generate_as_str_impl(&variants);
    let from_str_impl = generate_from_str_impl(&variants);
    let try_from_str_impl = generate_try_from_str_impl(&variants);
    let try_from_i32_impl = generate_try_from_i32_impl(&variants);
    let to_index_impl = generate_to_index_impl(&variants);
    let all_variants_impl = generate_all_variants_impl(&variants);
    let variant_count = variants.len();
    let expanded = quote! {
        impl #name {
            pub fn as_str(&self) -> &'static str {
                #as_str_impl
            }
            pub fn to_index(&self) -> i32 {
                #to_index_impl
            }
            pub const ALL:&'static[Self] = #all_variants_impl;
            pub const COUNT:usize = #variant_count;
        }

        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #from_str_impl
            }
        }
        impl TryFrom<&str> for #name {
            type Error = String;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                #try_from_str_impl
            }
        }

        impl TryFrom<i32> for #name {
            type Error = String;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                #try_from_i32_impl
            }
        }

        impl From<#name> for i32 {
            fn from(value: #name) -> Self {
                value.to_index()
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

fn generate_as_str_impl(variants: &[(&syn::Ident, i32, String, &Fields)]) -> TokenStream2 {
    let match_arms = variants
        .iter()
        .map(|(ident, _, desc, fields)| match fields {
            Fields::Unit => quote! { Self::#ident => #desc, },
            Fields::Unnamed(_) => quote! { Self::#ident(..) => #desc, },
            Fields::Named(_) => quote! { Self::#ident { .. } => #desc, },
        });

    quote! {
        match self {
            #(#match_arms)*
        }
    }
}

fn generate_from_str_impl(variants: &[(&syn::Ident, i32, String, &Fields)]) -> TokenStream2 {
    let match_arms = variants.iter().map(|(ident, _, desc, fields)| {
        match fields {
            Fields::Unit => quote! { #desc | stringify!(#ident) => Ok(Self::#ident), },
            _ => quote! { #desc | stringify!(#ident) => Err(format!("Cannot construct {} from string", #desc)), },
        }
    });

    quote! {
        match s {
            #(#match_arms)*
            _ => Err(format!("Invalid string: {}", s)),
        }
    }
}

fn generate_try_from_str_impl(variants: &[(&syn::Ident, i32, String, &Fields)]) -> TokenStream2 {
    let match_arms = variants.iter().map(|(ident, _, desc, fields)| {
        match fields {
            Fields::Unit => quote! { #desc | stringify!(#ident) => Ok(Self::#ident), },
            _ => quote! { #desc | stringify!(#ident) => Err(format!("Cannot construct {} from string", #desc)), },
        }
    });

    quote! {
        match s {
            #(#match_arms)*
            _ => Err(format!("Invalid string: {}", s)),
        }
    }
}

fn generate_try_from_i32_impl(variants: &[(&syn::Ident, i32, String, &Fields)]) -> TokenStream2 {
    let match_arms = variants.iter().map(|(ident, index, _, fields)| {
        match fields {
            Fields::Unit => quote! { #index => Ok(Self::#ident), },
            _ => quote! { #index => Err(format!("Cannot construct {} from i32", stringify!(#ident))), },
        }
    });

    quote! {
        match value {
            #(#match_arms)*
            _ => Err(format!("Invalid integer: {}", value)),
        }
    }
}

fn generate_to_index_impl(variants: &[(&syn::Ident, i32, String, &Fields)]) -> TokenStream2 {
    let match_arms = variants
        .iter()
        .map(|(ident, index, _, fields)| match fields {
            Fields::Unit => quote! { Self::#ident => #index, },
            Fields::Unnamed(f) => {
                let wildcards = std::iter::repeat(quote!(_)).take(f.unnamed.len());
                quote! { Self::#ident(#(#wildcards),*) => #index, }
            }
            Fields::Named(_) => quote! { Self::#ident { .. } => #index, },
        });

    quote! {
        match self {
            #(#match_arms)*
        }
    }
}
fn generate_all_variants_impl(variants: &[(&syn::Ident, i32, String, &Fields)]) -> TokenStream2 {
    let variant_arms = variants.iter().map(|(ident, _, _, fields)| match fields {
        Fields::Unit => quote! { Self::#ident, },
        Fields::Unnamed(_) | Fields::Named(_) => quote! {},
    });

    quote! {
        &[
            #(#variant_arms)*
        ]
    }
}
