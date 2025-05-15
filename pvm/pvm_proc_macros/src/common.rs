pub(crate) fn get_attr(attrs: &Vec<syn::Attribute>, attr_name: &str) -> Option<syn::Attribute> {
    for attr in attrs {
        if attr.path().is_ident(attr_name) {
            return Some(attr.clone());
        }
    }

    None
}
