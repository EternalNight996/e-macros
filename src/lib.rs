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

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Variant};

#[proc_macro_attribute]
pub fn value(_: TokenStream, item: TokenStream) -> TokenStream {
    let enum_input = parse_macro_input!(item as syn::ItemEnum);
    r#enum::create_structure(enum_input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

mod r#enum {
    use super::*;
    use syn::punctuated::Punctuated;

    pub(crate) fn variant_drives_impl(
        enum_name: &syn::Ident,
        variants: &mut Punctuated<Variant, syn::token::Comma>,
        repr_ty: &syn::Path,
    ) -> syn::ItemImpl {
        let mut variant_derive_value_expr: Vec<syn::Arm> = Vec::new();
        let mut variant_derive_index_expr: Vec<syn::Arm> = Vec::new();

        for variant in variants.iter_mut() {
            let ident = &variant.ident;
            let mut attrs_to_remove = Vec::new();

            for (i, attr) in variant.attrs.iter().enumerate() {
                if attr.path().is_ident("e") {
                    let mut value = None;
                    let mut index: Option<syn::Expr> = None;

                    let _ = attr.parse_nested_meta(|nv| {
                        if nv.path.is_ident("value") {
                            if let Ok(syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(v),
                                ..
                            })) = nv.value().and_then(|v| v.parse())
                            {
                                value = Some(v.value());
                            }
                        } else if nv.path.is_ident("index") {
                            index = nv.value().and_then(|v| v.parse()).ok();
                        }
                        Ok(())
                    });
                    if value.is_some() || index.is_some() {
                        attrs_to_remove.push(i);
                    }
                    if let Some(v) = value {
                        variant_derive_value_expr.push(parse_quote! {
                            Self::#ident => #v,
                        });
                    }

                    if let Some(idx) = index {
                        variant_derive_index_expr.push(parse_quote! {
                            Self::#ident => {
                                let index: #repr_ty = #idx;
                                index
                            },
                        });
                    }
                }
            }

            // Remove processed attributes
            for &i in attrs_to_remove.iter().rev() {
                variant.attrs.remove(i);
            }
        }

        parse_quote! {
            impl #enum_name {
                pub fn value(&self) -> &'static str {
                    match *self {
                        #(#variant_derive_value_expr)*
                        _ => "unknown",
                    }
                }
                pub fn index(&self) -> #repr_ty {
                    match *self {
                        #(#variant_derive_index_expr)*
                        _ => Default::default(),
                    }
                }
            }
        }
    }

    pub(crate) fn create_structure(enum_input: syn::ItemEnum) -> syn::Result<TokenStream2> {
        let enum_name = &enum_input.ident;
        let vis = &enum_input.vis;
        let mut variants = enum_input.variants;

        let (derive_and_repr_attrs, other_attrs): (Vec<_>, Vec<_>) = enum_input
            .attrs
            .into_iter()
            .partition(|attr| attr.path().is_ident("derive") || attr.path().is_ident("repr"));

        let (derive_attrs, repr_attrs): (Vec<_>, Vec<_>) = derive_and_repr_attrs
            .into_iter()
            .partition(|attr| attr.path().is_ident("derive"));

        let derive_items: Vec<syn::Path> = derive_attrs
            .iter()
            .flat_map(|attr| {
                attr.parse_args_with(Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
                    .unwrap_or_default()
            })
            .collect();

        let has_debug = derive_items.iter().any(|path| path.is_ident("Debug"));

        let repr_items: Vec<syn::Meta> = repr_attrs
            .iter()
            .flat_map(|attr| {
                attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                    .unwrap_or_default()
            })
            .collect();

        let (repr_ty, new_reprs) = crate::repr_ty(repr_items)?;

        let variant_drives_impl = variant_drives_impl(enum_name, &mut variants, &repr_ty);

        let display_impl = if has_debug {
            quote! {
                impl std::fmt::Display for #enum_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.value())
                    }
                }
            }
        } else {
            quote! {}
        };

        Ok(quote! {
            #(#other_attrs)*
            #(#[repr(#new_reprs)])*
            #[derive(#(#derive_items),*)]
            #vis enum #enum_name {
                #variants
            }

            #variant_drives_impl

            #display_impl
        })
    }
}

fn repr_ty(reprs: Vec<syn::Meta>) -> syn::Result<(syn::Path, Vec<syn::Meta>)> {
    let mut repr_ty = None;
    let mut new_reprs = Vec::new();

    for repr in reprs {
        if let syn::Meta::Path(path) = &repr {
            if [
                "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "isize", "usize",
            ]
            .iter()
            .any(|&t| path.is_ident(t))
            {
                repr_ty = Some(path.clone());
                new_reprs.push(repr.clone());
                continue;
            }
        }
        new_reprs.push(repr);
    }

    Ok((repr_ty.unwrap_or_else(|| parse_quote!(i32)), new_reprs))
}
