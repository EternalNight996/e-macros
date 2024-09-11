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
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, MetaList};

#[proc_macro_derive(Enum, attributes(descript, display))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let variants = if let Data::Enum(data_enum) = &input.data {
        data_enum
            .variants
            .iter()
            .map(|v| {
                let ident = &v.ident;
                let discriminant = if let Some((_, expr)) = &v.discriminant {
                    quote! { = #expr }
                } else {
                    quote! {}
                };

                let mut description = stringify!(#ident).to_string();
                for attr in &v.attrs {
                    let path = attr.path();
                    if path.is_ident("descript") {
                        if let Ok(tokens) = attr.parse_args_with(|input: syn::parse::ParseStream| {
                            let meta_list: MetaList = input.parse()?;
                            let tokens = meta_list.tokens.to_string();
                            println!("descript -> {:?}  ", tokens);
                            Ok(tokens)
                        }) {
                            description = tokens;
                        } else {
                            eprintln!("Failed to parse 'descript' attribute for variant {:?}", ident);
                        }
                    } else if path.is_ident("display") {
                        //
                    }
                }
                (ident, discriminant, description)
            })
            .collect::<Vec<_>>()
    } else {
        panic!("Enum derive macro can only be used with enums");
    };
    let variant_idents = variants.iter().map(|(ident, _, _)| ident);
    let variant_descriptions = variants.iter().map(|(_, _, description)| description);

    let expanded = quote! {
        impl #name {
            // pub fn as_str(&self) -> &'static str {
            //     match self {
            //         #(Self::#variant_idents => stringify!(#variant_idents),)*
            //     }
            // }

            // pub fn to_descript(&self) -> &'static str {
            //     match self {
            //         #(Self::#variant_idents => #variant_descriptions,)*
            //     }
            // }
        }

        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(stringify!(#variant_idents) => Ok(Self::#variant_idents),)*
                    _ => Err(format!("Invalid string: {}", s)),
                }
            }
        }

        // impl TryFrom<i32> for #name {
        //     type Error = String;

        //     fn try_from(value: i32) -> Result<Self, Self::Error> {
        //         match value {
        //             #(i if i == Self::#variant_idents as i32 => Ok(Self::#variant_idents),)*
        //             _ => Err(format!("Invalid integer: {}", value)),
        //         }
        //     }
        // }

        // impl From<#name> for i32 {
        //     fn from(value: #name) -> Self {
        //         value as i32
        //     }
        // }

        // impl std::fmt::Display for #name {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         write!(f, "{}", self.as_str())
        //     }
        // }

        // impl PartialEq for #name {
        //     fn eq(&self, other: &Self) -> bool {
        //         *self as i32 == *other as i32
        //     }
        // }

        // impl #name {
        //     pub fn from_descript(desc: &str) -> Result<Self, String> {
        //         match desc {
        //             #(#variant_descriptions => Ok(Self::#variant_idents),)*
        //             _ => Err(format!("Invalid description: {}", desc)),
        //         }
        //     }
        // }
    };

    TokenStream::from(expanded)
}
