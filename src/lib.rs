#![doc = include_str!("../README.md")]
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
use quote::{quote, ToTokens};
use std::collections::HashMap;

use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Token};

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
    let serde_json_impl = generate_serde_json_impl(name);

    let as_str_impl = generate_as_str_impl(&variants);
    let from_str_impl = generate_from_str_impl(&variants);
    let try_from_str_impl = generate_try_from_str_impl(&variants);
    let try_from_i32_impl = generate_try_from_i32_impl(&variants);
    let to_index_impl = generate_to_index_impl(&variants);
    let all_variants_impl = generate_all_variants_impl(&variants);
    let generate_as_variant = generate_as_variant_impl(&variants);
    let variant_count = variants.len();
    let expanded = quote! {
        impl #name {
            /// Returns the string representation of the enum variant.
            pub fn as_str(&self) -> &'static str {
                #as_str_impl
            }
            /// Returns the index of the enum variant.
            pub fn to_index(&self) -> i32 {
                #to_index_impl
            }
            /// Only ident
            pub fn as_variant(&self) -> &'static str {
                #generate_as_variant
            }
            /// Returns a slice containing all variants of the enum.
            pub const ALL: &'static [Self] = #all_variants_impl;
            /// Returns the count of all variants of the enum.
            pub const COUNT: usize = #variant_count;
            /// Returns the index of the enum variant.
            pub fn index(&self) -> i32 {
                self.to_index()
            }
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
        #serde_json_impl
    };

    TokenStream::from(expanded)
}

fn generate_serde_json_impl(name: &syn::Ident) -> TokenStream2 {
    quote! {
        #[cfg(feature = "serde_json")]
        impl From<&#name> for serde_json::Value {
            fn from(value: &#name) -> Self {
                serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
            }
        }

        #[cfg(feature = "serde_json")]
        impl TryFrom<serde_json::Value> for #name {
            type Error = serde_json::Error;

            fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
                serde_json::from_value(value)
            }
        }
    }
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

fn generate_as_variant_impl(variants: &[(&syn::Ident, i32, String, &Fields)]) -> TokenStream2 {
    let match_arms = variants
        .iter()
        .map(|(ident, _, _desc, fields)| match fields {
            Fields::Unit => quote! { Self::#ident => stringify!(#ident), },
            Fields::Unnamed(_) => quote! { Self::#ident(..) => stringify!(#ident), },
            Fields::Named(_) => quote! { Self::#ident { .. } => stringify!(#ident), },
        });

    quote! {
        match self {
            #(#match_arms)*
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

#[proc_macro_attribute]
pub fn value(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_input = parse_macro_input!(item as syn::ItemEnum);
    structify_impl(enum_input)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

fn structify_impl(r#enum: syn::ItemEnum) -> syn::Result<TokenStream2> {
    let enum_clone = r#enum.clone();
    let (reprs, derives, derive_debug, derive_default) = repr_derive(r#enum.attrs)?;

    let (variant_values, default_value, value_strings) = variants(r#enum.variants)?;

    let vis = r#enum.vis;
    let enum_name = r#enum.ident;
    let (repr_ty, reprs) = repr_ty(reprs)?;

    let mut token_stream = TokenStream2::new();
    let struct_item = structify_type(&reprs, &derives, &vis, &enum_name, &repr_ty);
    let inherent_impl = inherent_impl(&enum_name, &variant_values, &value_strings, &repr_ty);
    let from_impl = from_impl(&enum_name,&repr_ty);
    let phantom_enum = phantom_enum(enum_clone);

    struct_item.to_tokens(&mut token_stream);
    inherent_impl.to_tokens(&mut token_stream);
    from_impl.to_tokens(&mut token_stream);
    phantom_enum.to_tokens(&mut token_stream);

    if derive_debug {
        let debug_impl = debug_impl(&enum_name, &variant_values);
        debug_impl.to_tokens(&mut token_stream);
    }

    if derive_default {
        let default_impl = default_impl(&enum_name, &default_value);
        default_impl.to_tokens(&mut token_stream);
    }

    Ok(token_stream)
}

fn repr_derive(
    enum_attrs: Vec<syn::Attribute>,
) -> syn::Result<(Vec<syn::Meta>, Vec<syn::Path>, bool, bool)> {
    let mut reprs = vec![];
    let mut derives = vec![];
    let mut derive_debug = false;
    let mut derive_default = false;
    for attr in enum_attrs {
        if attr.path().is_ident("cfg") {
            continue;
        }

        if attr.path().is_ident("repr") {
            reprs.extend(
                attr.parse_args_with(Punctuated::<syn::Meta, Token![,]>::parse_terminated)?,
            );
            continue;
        }

        if attr.path().is_ident("derive") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("Clone")
                    || meta.path.is_ident("Copy")
                    || meta.path.is_ident("Eq")
                    || meta.path.is_ident("PartialEq")
                    || meta.path.is_ident("Ord")
                    || meta.path.is_ident("PartialOrd")
                    || meta.path.is_ident("Hash")
                {
                    derives.push(meta.path);
                    return Ok(());
                }

                if meta.path.is_ident("Debug") {
                    derive_debug = true;
                    return Ok(());
                }

                if meta.path.is_ident("Default") {
                    derive_default = true;
                    return Ok(());
                }

                Err(meta.error("unsupported derive. It only supports `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Default`, `Debug`."))
            })?;
            continue;
        }

        return Err(syn::Error::new(
            attr.span(),
            "unsupported attribute. It only supports `#[repr]` and `#[derive]`",
        ));
    }

    Ok((reprs, derives, derive_debug, derive_default))
}

fn repr_ty(reprs: Vec<syn::Meta>) -> syn::Result<(syn::Path, Vec<syn::Meta>)> {
    let mut repr_ty = None;
    let has_transparent = reprs.contains(&parse_quote!(transparent));
    let mut new_reprs = vec![];
    for repr in reprs {
        let syn::Meta::Path(path) = &repr else {
            new_reprs.push(repr);
            continue;
        };

        if path.is_ident("i8")
            || path.is_ident("u8")
            || path.is_ident("i16")
            || path.is_ident("u16")
            || path.is_ident("i32")
            || path.is_ident("u32")
            || path.is_ident("i64")
            || path.is_ident("u64")
            || path.is_ident("i128")
            || path.is_ident("u128")
            || path.is_ident("isize")
            || path.is_ident("usize")
        {
            if repr_ty.is_none() {
                if !has_transparent {
                    repr_ty = Some(path.clone());
                }
                continue;
            } else {
                return Err(syn::Error::new(
                    path.span(),
                    "conflicting representation hints",
                ));
            }
        }

        new_reprs.push(repr);
    }

    Ok((repr_ty.unwrap_or_else(|| parse_quote!(i32)), new_reprs))
}

fn variants(
    enum_variants: impl IntoIterator<Item = syn::Variant>,
) -> syn::Result<(
    HashMap<syn::Ident, syn::Expr>,
    syn::Expr,
    HashMap<syn::Ident, String>,
)> {
    let mut variant_values = HashMap::new();
    let mut value_strings = HashMap::new();
    let mut value: syn::Expr = parse_quote!(0);
    let mut default_value = None;
    for v in enum_variants {
        if !matches!(v.fields, syn::Fields::Unit) {
            return Err(syn::Error::new(
                v.span(),
                "unsupported variant. It only supports unit variant",
            ));
        }

        let mut value_string = None;
        for v_attr in &v.attrs {
            if v_attr.path().is_ident("value") {
                value_string = Some(v_attr.parse_args::<syn::LitStr>()?.value());
            }
        }

        if let Some((_, expr)) = v.discriminant {
            value = expr
        }

        if default_value.is_none() {
            let v_name = v.ident.clone();
            default_value = Some(parse_quote!(Self:: #v_name.0));
        }
        variant_values.insert(v.ident.clone(), value.clone());
        if let Some(value_str) = value_string {
            value_strings.insert(v.ident, value_str);
        }
        value = parse_quote! { #value + 1 };
    }

    Ok((
        variant_values,
        default_value.unwrap_or_else(|| parse_quote!(0)),
        value_strings,
    ))
}

fn structify_type(
    reprs: &[syn::Meta],
    derives: &[syn::Path],
    vis: &syn::Visibility,
    enum_name: &syn::Ident,
    repr_ty: &syn::Path,
) -> syn::ItemStruct {
    parse_quote! {
        #(#[repr(#reprs)])*
        #(#[derive(#derives)])*
        #vis struct #enum_name(#repr_ty);
    }
}

fn inherent_impl(
    enum_name: &syn::Ident,
    variant_values: &HashMap<syn::Ident, syn::Expr>,
    value_strings: &HashMap<syn::Ident, String>,
    repr_ty: &syn::Path,
) -> syn::ItemImpl {
    let const_variants: Vec<syn::ItemConst> = variant_values
        .iter()
        .map(|(v_name, value)| {
            parse_quote! {
                pub const #v_name: Self = Self(#value);
            }
        })
        .collect();

    let value_methods: Vec<syn::ImplItemFn> = value_strings
        .iter()
        .map(|(v_name, value_str)| {
            let value_str = value_str.clone();
            parse_quote! {
                pub fn value(&self) -> &'static str {
                    match self {
                        Self::#v_name => #value_str,
                        _ => "unknown",
                    }
                }
            }
        })
        .collect();

    parse_quote! {
        impl #enum_name {
            #(#const_variants)*

            pub const fn new(value: #repr_ty) -> Self {
                Self(value)
            }

            pub const fn value(&self) -> &'static str {
                match self.0 {
                    #(#value_methods)*
                    _ => "unknown",
                }
            }

            pub const fn index(&self) -> #repr_ty {
                self.0
            }
        }
    }
}

fn debug_impl(
    enum_name: &syn::Ident,
    variant_values: &HashMap<syn::Ident, syn::Expr>,
) -> syn::ItemImpl {
    let stmts: Vec<syn::ExprIf> = variant_values
        .keys()
        .map(|v_name| {
            parse_quote! {
                if self.0 == Self:: #v_name.0 {
                    return f.debug_struct(stringify!(#v_name)).finish();
                }
            }
        })
        .collect();

    parse_quote! {
        impl ::core::fmt::Debug for #enum_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #(#stmts)*

                f.debug_tuple(stringify!(#enum_name))
                    .field(&self.0)
                    .finish()
            }
        }
    }
}

fn default_impl(enum_name: &syn::Ident, default_value: &syn::Expr) -> syn::ItemImpl {
    parse_quote! {
        impl ::core::default::Default for #enum_name {
            fn default() -> Self {
                Self(#default_value)
            }
        }
    }
}

fn from_impl(enum_name: &syn::Ident, repr_ty: &syn::Path) -> TokenStream2 {
    quote! {
        impl ::core::convert::From<#repr_ty> for #enum_name {
            fn from(value: #repr_ty) -> Self {
                Self(value)
            }
        }

        impl ::core::convert::From<#enum_name> for #repr_ty {
            fn from(value: #enum_name) -> Self {
                value.0
            }
        }
    }
}

fn phantom_enum(mut r#enum: syn::ItemEnum) -> TokenStream2 {
    r#enum.attrs.clear();
    for v in r#enum.variants.iter_mut() {
        v.attrs.clear();
    }

    quote! {
        const _: () = {
            #r#enum
        };
    }
}
