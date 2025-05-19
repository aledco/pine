use proc_macro::TokenStream;
use quote::quote;

/// Derives the `TypedAst` trait for a struct.
pub(crate) fn derive_typed_ast(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let name = item_struct.ident.clone();
    quote! {
        impl TypedAst for #name {
            fn pine_type(&self) -> PineType {
                self.pine_type.clone()
            }

            fn set_pine_type(&mut self, pine_type: PineType) {
                self.pine_type = pine_type;
            }
        }
    }
        .into()
}