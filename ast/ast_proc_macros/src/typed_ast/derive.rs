use proc_macro::TokenStream;
use quote::quote;

/// Derives the `TypedAst` trait for a struct.
pub(crate) fn derive_typed_ast(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let name = item_struct.ident.clone();
    quote! {
        impl TypedAst for #name {
            fn ty(&self) -> PineType {
                self.ty.clone()
            }

            fn set_ty(&mut self, ty: PineType) {
                self.ty = ty;
            }
        }
    }
        .into()
}