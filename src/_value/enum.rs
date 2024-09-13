use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Fields, Variant};

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
        let mut value = None;
        let mut index: Option<syn::Expr> = None;
        for (i, attr) in variant.attrs.iter().enumerate() {
            if attr.path().is_ident("e") {
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
                attrs_to_remove.push(i);
            }
        }
        // Remove processed attributes
        for &i in attrs_to_remove.iter().rev() {
            variant.attrs.remove(i);
        }
        let value_expr = if let Some(v) = value {
            quote! { #v }
        } else {
            quote! { stringify!(#ident) }
        };

        match &variant.fields {
            syn::Fields::Unit => {
                variant_derive_value_expr.push(parse_quote! {
                    Self::#ident => #value_expr,
                });
            }
            _ => {
                variant_derive_value_expr.push(parse_quote! {
                    Self::#ident { .. } => #value_expr,
                });
            }
        }

        if let Some(idx) = index {
            match &variant.fields {
                syn::Fields::Unit => {
                    variant_derive_index_expr.push(parse_quote! {
                        Self::#ident => {
                            let index: #repr_ty = #idx;
                            index
                        },
                    });
                }
                _ => {
                    variant_derive_index_expr.push(parse_quote! {
                        Self::#ident { .. } => {
                            let index: #repr_ty = #idx;
                            index
                        },
                    });
                }
            }
        }
    }

    parse_quote! {
        impl #enum_name {
            pub fn value(&self) -> &'static str {
                match *self {
                    #(#variant_derive_value_expr)*
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

    let (derive_attrs, repr_attrs, other_attrs) = split_attributes(enum_input.attrs);
    let (has_serialize, has_deserialize, derive_items) = process_derive_attrs(derive_attrs);
    let has_debug = derive_items.iter().any(|path| path.is_ident("Debug"));

    let repr_items = parse_repr_attrs(repr_attrs);
    let (repr_ty, new_reprs) = super::repr_ty(repr_items)?;

    let variant_drives_impl = variant_drives_impl(enum_name, &mut variants, &repr_ty);
    let display_impl = generate_display_impl(enum_name, has_debug);

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

fn split_attributes(
    attrs: Vec<syn::Attribute>,
) -> (
    Vec<syn::Attribute>,
    Vec<syn::Attribute>,
    Vec<syn::Attribute>,
) {
    let (derive_and_repr_attrs, other_attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path().is_ident("derive") || attr.path().is_ident("repr"));

    let (derive_attrs, repr_attrs): (Vec<_>, Vec<_>) = derive_and_repr_attrs
        .into_iter()
        .partition(|attr| attr.path().is_ident("derive"));

    (derive_attrs, repr_attrs, other_attrs)
}

fn process_derive_attrs(derive_attrs: Vec<syn::Attribute>) -> (bool, bool, Vec<syn::Path>) {
    let derive_items: Vec<syn::Path> = derive_attrs
        .iter()
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
                .unwrap_or_default()
        })
        .collect();

    derive_items
        .into_iter()
        .fold((false, false, Vec::new()), |(ser, de, mut items), path| {
            let is_serialize = path.is_ident("Serialize")
                || path
                    .segments
                    .last()
                    .map_or(false, |seg| seg.ident == "Serialize");
            let is_deserialize = path.is_ident("Deserialize")
                || path
                    .segments
                    .last()
                    .map_or(false, |seg| seg.ident == "Deserialize");

            items.push(path.clone());
            (ser || is_serialize, de || is_deserialize, items)
        })
}

fn parse_repr_attrs(repr_attrs: Vec<syn::Attribute>) -> Vec<syn::Meta> {
    repr_attrs
        .iter()
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .unwrap_or_default()
        })
        .collect()
}

fn generate_display_impl(enum_name: &syn::Ident, has_debug: bool) -> TokenStream2 {
    if has_debug {
        quote! {
            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.value())
                }
            }
        }
    } else {
        quote! {}
    }
}
