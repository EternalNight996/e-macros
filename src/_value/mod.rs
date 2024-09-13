use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Variant};

pub(crate) mod r#enum;

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
        // 如果没有指定 repr 但有显式枚举值，返回默认的 i32
        return Ok((parse_quote!(i32), quote! { #[repr(i32)] }));
    }

    if reprs.is_empty() {
        // 如果没有指定任何 repr 且没有显式枚举值，返回空的 TokenStream
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
