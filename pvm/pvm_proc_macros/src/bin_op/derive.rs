use proc_macro::TokenStream;
use quote::quote;
use crate::common;
use crate::bin_op::BinOpAttributes;

/// Implements the `BinOpInst` derive macro.
pub(crate) fn derive_bin_op_inst(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &item_struct.ident.clone();

    if let Some(bin_op_attr) = common::get_attr(&item_struct.attrs, "bin_op_helper") {
        let mut attrs = BinOpAttributes::default();
        bin_op_attr.parse_nested_meta(|meta| attrs.parse(meta)).unwrap();
        let operator = attrs.operator.unwrap();
        let val1_ty = attrs.val1_ty.unwrap();
        let val2_ty = attrs.val2_ty.unwrap();

        return quote! {
            impl Instruction for #struct_name {
                fn execute(&mut self, env: &mut Environment) -> Result<(), crate::error::Error> {
                    let val1 = crate::cast::from_u64!(self.src1.value(env)?; #val1_ty);
                    let val2 = crate::cast::from_u64!(self.src2.value(env)?; #val2_ty);
                    let res = crate::cast::to_u64!(#operator(val1, val2));
                    self.dest.set_value(res, env)?;
                    Ok(())
                }
            }

            impl std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} {} {} {}", Self::NAME, self.dest, self.src1, self.src2)
                }
            }
        }.into();
    }

    panic!("bin_op_helper attribute is required");
}