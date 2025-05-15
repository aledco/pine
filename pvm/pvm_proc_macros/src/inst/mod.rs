mod attr;
mod derive;
pub(crate) use attr::*;
pub(crate) use derive::*;

/// The `inst` macro attributes.
#[derive(Default)]
pub(crate) struct InstAttributes {
    pub inst_name: Option<syn::LitStr>,
    pub operands: Option<syn::ExprArray>
}

impl InstAttributes {
    /// Parses the `inst` macro attributes.
    pub(crate) fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> Result<(), syn::Error> {
        if meta.path.is_ident("name") {
            self.inst_name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("operands") {
            self.operands = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("Unrecognized inst argument"))
        }
    }
}




