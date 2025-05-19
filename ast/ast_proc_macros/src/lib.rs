mod ast;
mod typed_ast;

extern crate proc_macro;
use proc_macro::TokenStream;

/// Implements the `ast` proc macro.
#[proc_macro_attribute]
pub fn ast(args: TokenStream, input: TokenStream) -> TokenStream {
    ast::ast_attr(args, input)
}

/// Implements the `typed_ast` proc macro.
#[proc_macro_attribute]
pub fn typed_ast(args: TokenStream, input: TokenStream) -> TokenStream {
    typed_ast::typed_ast_attr(args, input)
}

/// Derives the `Ast` trait for a struct.
#[proc_macro_derive(Ast)]
pub fn derive_ast_node(input: TokenStream) -> TokenStream {
    ast::derive_ast_node(input)
}

/// Derives the `TypedAst` trait for a struct.
#[proc_macro_derive(TypedAst)]
pub fn derive_typed_ast(input: TokenStream) -> TokenStream {
    typed_ast::derive_typed_ast(input)
}

/// Derives the `new` function for a struct.
#[proc_macro_derive(NewAst, attributes(default))]
pub fn derive_new_ast(input: TokenStream) -> TokenStream {
    ast::derive_new_ast(input)
}
