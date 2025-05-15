use proc_macro::TokenStream;
use quote::quote;
use crate::common;

/// Implements the `PrintInst` derive macro.
pub(crate) fn derive_print_inst(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &item_struct.ident.clone();

    if let Some(print_attr) = common::get_attr(&item_struct.attrs, "print_helper") {

        let ty: syn::Type = print_attr.parse_args().unwrap();

        return quote! {
            impl Instruction for #struct_name {
                fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
                    let value: #ty = crate::from_u64!(self.src.value(env)?; #ty);
                    let res = write!(env.stdout.borrow_mut(), "{}", value);
                    match res {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("{}", e)),
                    }
                }
            }

            impl Display for #struct_name {
                fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, "{} {}", Self::NAME, self.src)
                }
            }
        }.into();
    }

    panic!("print_helper attribute is required");
}