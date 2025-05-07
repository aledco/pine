extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::default::Default;
use syn::{parse, parse::Parser, parse_macro_input, Attribute, Expr, ExprCall, ExprReturn, ExprStruct, FieldValue, FieldsNamed, FnArg, ItemFn, ItemStruct, Meta};

#[proc_macro_attribute]
pub fn ast(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        add_ast_fields(fields);
    }

    let derive_attr: Attribute = syn::parse_quote! { #[derive(Ast, NewAst, Debug)] };
    item_struct.attrs.push(derive_attr);

    quote! {
        #item_struct
    }
    .into()
}

// #[proc_macro_attribute]
// pub fn statement(args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut item_struct = parse_macro_input!(input as ItemStruct);
//     let _ = parse_macro_input!(args as parse::Nothing);
//
//     if let syn::Fields::Named(ref mut fields) = item_struct.fields {
//         add_ast_fields(fields);
//     }
//
//     let derive_quote = quote! { #[derive(Ast, Statement, Debug)] };
//     let derive_attr: Attribute = syn::parse_quote!(#derive_quote);
//     item_struct.attrs.push(derive_attr);
//
//     quote! {
//         #item_struct
//     }.into()
// }

#[proc_macro_attribute]
pub fn typed_ast(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        add_ast_fields(fields);

        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pine_type: PineType })
                .unwrap(),
        );
    }

    let derive_attr: Attribute = syn::parse_quote!{ #[derive(Ast, TypedAst, NewAst, Debug)] };
    item_struct.attrs.push(derive_attr);

    quote! {
        #item_struct
    }
    .into()
}

#[proc_macro_derive(Ast)]
pub fn derive_ast_node(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let name = item_struct.ident.clone();
    quote! {
        impl Ast for #name {
            fn scope(&self) -> ScopeRef {
                self.scope.clone()
            }

            fn set_scope(&mut self, scope: ScopeRef) {
                self.scope = scope;
            }

            fn span(&self) -> &Span {
                &self.span
            }
        }
    }
    .into()
}

// #[proc_macro_derive(Statement)]
// pub fn derive_statement(input: TokenStream) -> TokenStream {
//     let item_struct = parse_macro_input!(input as ItemStruct);
//     let name = item_struct.ident.clone();
//     quote! {
//         impl Statement for #name { }
//     }.into()
// }

#[proc_macro_derive(TypedAst)]
pub fn derive_typed_ast(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
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

#[proc_macro_derive(NewAst, attributes(omit_from_new))]
pub fn derive_new_ast(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    create_new_fn(&item_struct)
}

fn add_ast_fields(fields: &mut FieldsNamed) {
    fields.named.push(
        syn::Field::parse_named
            .parse2(quote! { #[omit_from_new(default = Scope::default())] scope: ScopeRef })
            .unwrap(),
    );

    fields.named.push(
        syn::Field::parse_named
            .parse2(quote! { span: Span })
            .unwrap(),
    );
}

fn create_new_fn(item_struct: &ItemStruct) -> TokenStream {
    let struct_name = &item_struct.ident.clone();

    let mut new_fn: ItemFn = syn::parse_quote! {
        pub fn new() -> Self {

        }
    };

    let mut self_expr: ExprStruct = syn::parse_quote! { Self {} };

    if let syn::Fields::Named(ref fields) = item_struct.fields {
        for f in &fields.named {
            let name = f.ident.clone();
            let ty = f.ty.clone();
            let attrs = f.attrs.clone();

            for a in attrs {
                if a.path().is_ident("omit_from_new") {
                    let mut default_constructor = None::<ExprCall>;
                    a.parse_nested_meta(|meta| {
                            if meta.path.is_ident("default") {
                                let value = meta.value()?;
                                let call_expr: ExprCall = value.parse()?;
                                default_constructor = Some(call_expr);
                                return Ok(());
                            }

                            return Ok(());
                        }
                    ).unwrap();
                    
                    if let Some(default_constructor) = default_constructor {
                        let value_quote = quote! { #name: #default_constructor };
                        let value: FieldValue = syn::parse_quote!(#value_quote);
                        self_expr.fields.push(value);
                    } else {
                        panic!();
                    }
                    
                    break;
                }
            }

            // if f.ident.clone().unwrap().to_string() == "scope" {
            //     // TODO use field attribute instead
            //     let value_quote = quote! { scope: Scope::default() };
            //     let value: FieldValue = syn::parse_quote!(#value_quote);
            //     self_expr.fields.push(value);
            //     continue;
            // }


            // add the function argument
            let fn_arg_quote = quote! { #name: #ty };
            let fn_arg: FnArg = syn::parse_quote!(#fn_arg_quote);
            new_fn.sig.inputs.push(fn_arg);

            // add the initalizer to the struct
            let value_quote = quote! { #name };
            let value: FieldValue = syn::parse_quote!(#value_quote);
            self_expr.fields.push(value);
        }
    }

    let return_expr = Expr::Return(ExprReturn {
        attrs: vec![],
        return_token: Default::default(),
        expr: Some(Box::new(Expr::Struct(self_expr))),
    });
    let return_stmt = syn::Stmt::Expr(return_expr, None);
    new_fn.block.stmts.push(return_stmt);

    quote! {
        impl #struct_name {
            #new_fn
        }
    }
    .into()
}
