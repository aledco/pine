mod attr;
mod derive;

use quote::quote;
use syn::parse::Parser;
pub(crate) use attr::*;
pub(crate) use derive::*;

/// Adds the common fields `scope` and `span` to an AST struct.
pub(crate) fn add_ast_fields(fields: &mut syn::FieldsNamed) {
    fields.named.push(
        syn::Field::parse_named
            .parse2(quote! { #[default(Scope::default)] scope: ScopeRef })
            .unwrap(),
    );

    fields.named.push(
        syn::Field::parse_named
            .parse2(quote! { span: Span })
            .unwrap(),
    );
}
