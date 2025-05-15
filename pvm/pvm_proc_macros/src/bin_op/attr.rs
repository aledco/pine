use crate::bin_op::BinOpAttributes;
use crate::{common, inst};
use proc_macro::TokenStream;
use quote::quote;

/// Implements the `bin_op` macro.
pub(crate) fn bin_op_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = syn::parse_macro_input!(input as syn::ItemStruct);

    let mut attrs = inst::InstAttributes::default();
    if let Some(inst_attr) = common::get_attr(&item_struct.attrs, "inst_helper") {
        inst_attr
            .parse_nested_meta(|meta| attrs.parse(meta))
            .unwrap();
    } else {
        panic!()
    }

    let mut attrs = BinOpAttributes::default();
    let bin_op_parser = syn::meta::parser(|meta| attrs.parse(meta));
    syn::parse_macro_input!(args with bin_op_parser);

    let op = attrs.operator.unwrap();
    let ty1 = attrs.val1_ty.unwrap_or_else(|| syn::parse_quote!(u64));
    let ty2 = attrs.val2_ty.unwrap_or_else(|| syn::parse_quote!(u64));

    let helper_attr: syn::Attribute =
        syn::parse_quote! { #[bin_op_helper(op = #op, ty1 = #ty1, ty2 = #ty2)] };
    item_struct.attrs.insert(0, helper_attr);
    let derive_attr: syn::Attribute = syn::parse_quote! { #[derive(BinOpInst)] };
    item_struct.attrs.insert(0, derive_attr);

    quote! {
        #item_struct
    }
    .into()
}
