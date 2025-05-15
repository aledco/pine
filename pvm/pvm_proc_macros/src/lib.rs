mod inst;
mod common;
mod bin_op;
mod print;

extern crate proc_macro;
use proc_macro::TokenStream;

/// The `inst` proc macro.
#[proc_macro_attribute]
pub fn inst(args: TokenStream, input: TokenStream) -> TokenStream {
    inst::inst_attr(args, input)
}

/// The `bin_op` proc macro.
#[proc_macro_attribute]
pub fn bin_op(args: TokenStream, input: TokenStream) -> TokenStream {
    bin_op::bin_op_attr(args, input)
}

/// The `print` proc macro.
#[proc_macro_attribute]
pub fn print(args: TokenStream, input: TokenStream) -> TokenStream {
    print::print_attr(args, input)
}

/// The `Inst` derive macro.
#[proc_macro_derive(Inst, attributes(inst_helper))]
pub fn derive_inst(input: TokenStream) -> TokenStream {
    inst::derive_inst(input)
}

/// The `NewInst` derive macro.
#[proc_macro_derive(NewInst)]
pub fn derive_new_inst(input: TokenStream) -> TokenStream {
    inst::derive_new_inst(input)
}

/// The `BinOpInst` derive macro.
#[proc_macro_derive(BinOpInst, attributes(bin_op_helper))]
pub fn derive_bin_op_inst(input: TokenStream) -> TokenStream {
    bin_op::derive_bin_op_inst(input)
}

/// The `PrintInst` derive macro.
#[proc_macro_derive(PrintInst, attributes(print_helper))]
pub fn derive_print_inst(input: TokenStream) -> TokenStream {
    print::derive_print_inst(input)
}
