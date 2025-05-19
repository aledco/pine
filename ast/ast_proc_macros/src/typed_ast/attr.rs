use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;

/// Implements the `typed_ast` proc macro.
pub(crate) fn typed_ast_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let _ = syn::parse_macro_input!(args as syn::parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        crate::ast::add_ast_fields(fields);

        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { #[default(PineType::default)] pine_type: PineType })
                .unwrap(),
        );
    }

    let derive_attr: syn::Attribute = syn::parse_quote! { #[derive(Ast, TypedAst, NewAst, Debug)] };
    item_struct.attrs.push(derive_attr);

    quote! {
        #item_struct
    }
        .into()
}