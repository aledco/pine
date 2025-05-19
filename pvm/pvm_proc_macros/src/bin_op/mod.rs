mod attr;
mod derive;
pub(crate) use attr::*;
pub(crate) use derive::*;

/// The `bin_op` macro attributes.
#[derive(Default)]
pub(crate) struct BinOpAttributes {
    pub operator: Option<syn::Path>,
    pub val1_ty: Option<syn::Type>,
    pub val2_ty: Option<syn::Type>
}

impl BinOpAttributes {
    /// Parses the `bin_op` macro attributes.
    pub(crate) fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> Result<(), syn::Error> {
        if meta.path.is_ident("op") {
            self.operator = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("ty1") {
            self.val1_ty = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("ty2") {
            self.val2_ty = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("Unrecognized bin_op argument"))
        }
    }
}
