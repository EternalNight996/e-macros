use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated};

/// Creates the structure for the enhanced enum.
///
/// This function generates the main implementation for the enum, including:
/// - Attribute processing
/// - Variant implementations
/// - Display and Serde implementations
pub(crate) fn create_structure(enum_input: syn::ItemEnum) -> syn::Result<TokenStream2> {
    let enum_name = enum_input.ident;
    let vis = enum_input.vis;
    let attrs = enum_input.attrs;
    let mut variants = enum_input.variants;

    // Split attributes into derive, repr, and other attributes
    let (derive_attrs, repr_attrs, other_attrs) = split_attributes(attrs);

    // Process derive attributes to determine which traits are derived
    let (has_debug, has_serialize, has_deserialize, derive_items) =
        process_derive_attrs(derive_attrs);

    // Get representation type and new repr attributes
    let (repr_ty, new_reprs) = super::repr_ty(repr_attrs, &variants)?;

    // Generate implementations for variants
    let variant_drives_impl = variant_drives_impl(&enum_name, &mut variants, &repr_ty);

    // Generate Display implementation if Debug is derived
    let display_impl = generate_display_impl(&enum_name, has_debug);

    // Generate Serde implementation if Serialize or Deserialize is derived
    let serde_impl = serde_impl(&enum_name, has_serialize, has_deserialize);

    // Combine all generated code into final implementation
    Ok(quote! {
        #(#other_attrs)*
        #new_reprs
        #[derive(#(#derive_items),*)]
        #vis enum #enum_name {
            #variants
        }

        #variant_drives_impl

        #display_impl

        #serde_impl
    })
}

/// Generates implementations for enum variants.
///
/// This function creates:
/// - TryFrom<repr_ty> implementation
/// - TryFrom<&str> implementation
/// - value(), index(), and variant_count() methods
pub(crate) fn variant_drives_impl(
    enum_name: &syn::Ident,
    variants: &mut Punctuated<syn::Variant, syn::token::Comma>,
    repr_ty: &syn::Path,
) -> TokenStream2 {
    let mut variant_derive_value_expr: Vec<syn::Arm> = Vec::new();
    let mut variant_derive_index_expr: Vec<syn::Arm> = Vec::new();
    let mut variant_derive_from_expr: Vec<syn::Arm> = Vec::new();
    let mut variant_derive_from_str_expr: Vec<TokenStream2> = Vec::new();
    let mut last_index: syn::Expr = parse_quote!(0 as #repr_ty);

    // Process each variant
    for variant in variants.iter_mut() {
        let ident = &variant.ident;
        let mut attrs_to_remove = Vec::new();
        let mut value = None;
        let mut index: Option<syn::Expr> = None;

        // Extract custom attributes (e.g., value and index)
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

        // Generate value expression
        let value_expr = if let Some(v) = value {
            quote! { #v }
        } else {
            quote! { stringify!(#ident) }
        };

        // Generate match arms for value, index, and from implementations
        match &variant.fields {
            syn::Fields::Unit => {
                variant_derive_value_expr.push(parse_quote! {
                    Self::#ident => #value_expr,
                });
                variant_derive_from_str_expr.push(match &variant.fields {
                    syn::Fields::Unit => quote! {
                        #value_expr => Ok(Self::#ident),
                    },
                    syn::Fields::Named(fields) => {
                        let field_inits = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            quote! { #name: Default::default() }
                        });
                        quote! {
                            #value_expr => Ok(Self::#ident { #(#field_inits),* }),
                        }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let field_inits =
                            (0..fields.unnamed.len()).map(|_| quote! { Default::default() });
                        quote! {
                            #value_expr => Ok(Self::#ident(#(#field_inits),*)),
                        }
                    }
                });
            }
            syn::Fields::Named(_) => {
                variant_derive_value_expr.push(parse_quote! {
                    Self::#ident { .. } => #value_expr,
                });
            }
            syn::Fields::Unnamed(_) => {
                variant_derive_value_expr.push(parse_quote! {
                    Self::#ident(..) => #value_expr,
                });
            }
        }

        // Generate index expression
        let idx = if let Some(idx) = index {
            last_index = parse_quote!(#idx);
            idx
        } else {
            last_index = parse_quote! { match (#last_index as #repr_ty).checked_add(1) {
                Some(next_index) => next_index,
                None => {
                    eprintln!("Index overflow: enum {} index exceeds the range of {}", stringify!(#enum_name), stringify!(#repr_ty));
                    #last_index
                }
            }};
            last_index.clone()
        };

        // Generate match arms for index and from implementations
        match &variant.fields {
            syn::Fields::Unit => {
                variant_derive_index_expr.push(parse_quote! {
                    Self::#ident => #idx,
                });
                variant_derive_from_expr.push(parse_quote! {
                    value if value == #idx => Ok(Self::#ident),
                });
            }
            syn::Fields::Named(_) => {
                variant_derive_index_expr.push(parse_quote! {
                    Self::#ident { .. } => #idx,
                });
            }
            syn::Fields::Unnamed(_) => {
                variant_derive_index_expr.push(parse_quote! {
                    Self::#ident(..) => #idx,
                });
            }
        }
    }

    let variant_count = variants.len();

    // Generate TryFrom<repr_ty> implementation
    let from_impl = quote! {
        impl TryFrom<#repr_ty> for #enum_name {
            type Error = &'static str;

            fn try_from(value: #repr_ty) -> Result<Self, Self::Error> {
                match value {
                    #(#variant_derive_from_expr)*
                    _ => Err(concat!("Invalid value ", stringify!(#repr_ty), " for enum \"", stringify!(#enum_name), "\"")),
                }
            }
        }
    };

    // Generate TryFrom<&str> implementation
    let from_str_impl = quote! {
        impl TryFrom<&str> for #enum_name {
            type Error = &'static str;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    #(#variant_derive_from_str_expr)*
                    _ => Err(concat!("Invalid string value for enum \"", stringify!(#enum_name), "\"")),
                }
            }
        }
    };

    // Combine all implementations
    quote! {
        #from_impl

        #from_str_impl

        impl #enum_name {
            /// # Returns the string value of the enum variant.
            /// # Example
            /// ```rust
            ///
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
            pub fn value(&self) -> &'static str {
                match self {
                    #(#variant_derive_value_expr)*
                }
            }

            /// #Returns the index value of the enum variant.
            /// # Example
            /// ```rust
            /// #[e_macros::value]
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
            pub fn index(&self) -> #repr_ty {
                match self {
                    #(#variant_derive_index_expr)*
                    _ => <#repr_ty>::default(),
                }
            }

            /// #Returns the number of variants in the enum.
            /// # Example
            /// ```rust
            ///
            /// #[e_macros::value]
            /// enum Color {
            ///     Red,
            ///     Green,
            ///     Blue,
            /// }
            /// fn main() {
            ///     println!("Cariant len: {}", Color::variant_count());
            /// }
            /// ```
            pub fn variant_count() -> usize {
                #variant_count
            }
        }
    }
}

/// Generates Serde-related implementations if Serialize or Deserialize is derived.
fn serde_impl(enum_name: &syn::Ident, has_serialize: bool, has_deserialize: bool) -> TokenStream2 {
    let serialize_impl = if has_serialize {
        quote! {
            pub fn to_serde(&self) -> Result<String, serde_json::Error> {
                serde_json::to_string(&self)
            }
        }
    } else {
        quote! {}
    };

    let deserialize_impl = if has_deserialize {
        quote! {
            pub fn from_serde(value: serde_json::Value) -> Result<Self, serde_json::Error> {
                serde_json::from_value(value)
            }
        }
    } else {
        quote! {}
    };

    quote! {
        impl #enum_name {
            #serialize_impl
            #deserialize_impl
        }
    }
}

/// Splits attributes into derive, repr, and other attributes.
fn split_attributes(
    attrs: Vec<syn::Attribute>,
) -> (
    Vec<syn::Attribute>,
    Vec<syn::Attribute>,
    Vec<syn::Attribute>,
) {
    let mut derive_attrs = Vec::new();
    let mut repr_attrs = Vec::new();
    let mut other_attrs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("derive") {
            derive_attrs.push(attr);
        } else if attr.path().is_ident("repr") {
            repr_attrs.push(attr);
        } else {
            other_attrs.push(attr);
        }
    }

    (derive_attrs, repr_attrs, other_attrs)
}

/// Processes derive attributes to determine which traits are derived.
fn process_derive_attrs(derive_attrs: Vec<syn::Attribute>) -> (bool, bool, bool, Vec<syn::Path>) {
    let mut has_debug = false;
    let mut has_serialize = false;
    let mut has_deserialize = false;
    let mut derive_items = Vec::new();

    for attr in derive_attrs {
        if let Ok(nested) = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        ) {
            for path in nested {
                if path.is_ident("Debug") {
                    has_debug = true;
                } else if path.is_ident("Serialize") {
                    has_serialize = true;
                } else if path.is_ident("Deserialize") {
                    has_deserialize = true;
                }
                derive_items.push(path);
            }
        }
    }

    (has_debug, has_serialize, has_deserialize, derive_items)
}

/// Generates Display implementation if Debug is derived.
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
