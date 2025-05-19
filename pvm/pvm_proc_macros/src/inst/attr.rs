use proc_macro::TokenStream;
use quote::quote;
use crate::inst::InstAttributes;

/// Implements the `inst` macro.
pub(crate) fn inst_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = syn::parse_macro_input!(input as syn::ItemStruct);

    // parse args
    let mut attrs = InstAttributes::default();
    let inst_parser = syn::meta::parser(|meta| attrs.parse(meta));
    syn::parse_macro_input!(args with inst_parser);

    let inst_name = attrs.inst_name.unwrap().value();
    if inst_name.is_empty() {
        panic!("Instruction name cannot be empty");
    }

    let operands: syn::ExprArray = attrs.operands.unwrap();

    let helper_attr: syn::Attribute = syn::parse_quote! { #[inst_helper(name = #inst_name, operands = #operands)] };
    item_struct.attrs.insert(0, helper_attr);
    let derive_attr: syn::Attribute = syn::parse_quote! { #[derive(Inst, std::fmt::Debug)] };
    item_struct.attrs.insert(0, derive_attr);
    let new_derive_attr: syn::Attribute = syn::parse_quote! { #[derive(NewInst)] };
    item_struct.attrs.push(new_derive_attr);
    quote! {
        #item_struct
    }.into()
}