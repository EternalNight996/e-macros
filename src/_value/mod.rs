use syn::parse_quote;

pub(crate) mod r#enum;


pub(crate) fn repr_ty(reprs: Vec<syn::Meta>) -> syn::Result<(syn::Path, Vec<syn::Meta>)> {
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
