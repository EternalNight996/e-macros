use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Variant};

pub(crate) mod r#enum;

/// Determines the representation type for the enum based on attributes and variants.
///
/// This function analyzes the `repr` attributes and enum variants to decide on an appropriate
/// representation type for the enum.
pub(crate) fn repr_ty(
    repr_attrs: Vec<syn::Attribute>,
    variants: &Punctuated<Variant, syn::token::Comma>,
) -> syn::Result<(syn::Path, TokenStream2)> {
    let reprs: Vec<syn::Meta> = repr_attrs
        .iter()
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .unwrap_or_default()
        })
        .collect();

    let has_explicit_discriminants = variants.iter().any(|v| v.discriminant.is_some());

    if reprs.is_empty() && has_explicit_discriminants {
        // If no repr is specified but there are explicit enum values, return default i32
        return Ok((parse_quote!(i32), quote! { #[repr(i32)] }));
    }

    if reprs.is_empty() {
        // If no repr is specified and there are no explicit enum values, return an empty TokenStream
        return Ok((parse_quote!(i32), TokenStream2::new()));
    }

    let valid_int_reprs = [
        "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128", "isize", "usize",
    ];

    let repr_ty = reprs.iter().find_map(|repr| {
        if let syn::Meta::Path(path) = repr {
            if path.is_ident("C") {
                Some(parse_quote!(::core::primitive::u32))
            } else if valid_int_reprs.iter().any(|&t| path.is_ident(t)) {
                Some(path.clone())
            } else {
                None
            }
        } else {
            None
        }
    });

    let repr_ty = repr_ty.unwrap_or_else(|| parse_quote!(::core::primitive::u32));
    let repr_attr = quote! { #(#[repr(#reprs)])* };

    Ok((repr_ty, repr_attr))
}
