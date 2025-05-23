use proc_macro::TokenStream;
use quote::quote;

/// Implements the `ast` proc macro.
pub(crate) fn ast_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let _ = syn::parse_macro_input!(args as syn::parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        crate::ast::add_ast_fields(fields);
    }

    let derive_attr: syn::Attribute = syn::parse_quote! { #[derive(Ast, NewAst, Debug)] };
    item_struct.attrs.push(derive_attr);

    quote! {
        #item_struct
    }
        .into()
}
