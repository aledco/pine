use proc_macro::TokenStream;
use quote::quote;
use crate::{common, inst};

/// Implements the `print` macro.
pub(crate) fn print_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = syn::parse_macro_input!(input as syn::ItemStruct);

    let mut attrs = inst::InstAttributes::default();
    if let Some(inst_attr) = common::get_attr(&item_struct.attrs, "inst_helper") {
        inst_attr.parse_nested_meta(|meta| attrs.parse(meta)).unwrap();
    } else {
        panic!()
    }

    let ty: syn::Type = syn::parse_macro_input!(args);
    let helper_attr: syn::Attribute = syn::parse_quote! { #[print_helper(#ty)] };
    item_struct.attrs.insert(0, helper_attr);
    let derive_attr: syn::Attribute = syn::parse_quote! { #[derive(PrintInst)] };
    item_struct.attrs.insert(0, derive_attr);

    quote! {
        #item_struct
    }.into()
}
