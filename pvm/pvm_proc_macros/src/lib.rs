extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse, parse::Parser, parse_macro_input, Attribute, Expr, ExprCall, ExprReturn, ExprStruct,
    Field, FieldValue, FieldsNamed, FnArg, ItemFn, ItemStruct, LitInt,
};

#[proc_macro_attribute]
pub fn arithmetic(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    let ops_token: LitInt = parse_macro_input!(args);
    let ops: u32 = ops_token.base10_parse().unwrap();
    //let op: syn::BinOp = parse_macro_input!(args); TODO parse operator
    if ops < 1 || ops > 2 {
        panic!("Arithmetic instruction can only have 1 or 2 operands");
    }

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields
            .named
            .push(Field::parse_named.parse2(quote! { #[variable] dest: Operand }).unwrap());

        if ops == 1 { // TODO add way to specify that it cannot be a label too (maybe allowed and not_allowed attributes with args instead)
            fields
                .named
                .push(Field::parse_named.parse2(quote! { src: Operand }).unwrap());
        } else if ops == 2 {
            fields
                .named
                .push(Field::parse_named.parse2(quote! { src1: Operand }).unwrap());

            fields
                .named
                .push(Field::parse_named.parse2(quote! { src2: Operand }).unwrap());
        }
    }

    let derive_attr: Attribute = syn::parse_quote! { #[derive(NewInst, Debug)] };
    item_struct.attrs.push(derive_attr);

    quote! {
        #item_struct
    }
    .into()
}

#[proc_macro_derive(NewInst, attributes(constant, variable, label))]
pub fn derive_new_ast(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    create_new_fn(&item_struct)
}

fn has_attr(field: &Field, attr_name: &str) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident(attr_name) {
            return true
        }
    }

    false
}

fn create_new_fn(item_struct: &ItemStruct) -> TokenStream {
    let struct_name = &item_struct.ident.clone();

    let mut new_fn: ItemFn = syn::parse_quote! {
        pub fn new() -> Self {

        }
    };

    let mut validation_stmt: Vec<syn::Stmt> = Vec::new();
    let mut self_expr: ExprStruct = syn::parse_quote! { Self {} };
    if let syn::Fields::Named(ref fields) = item_struct.fields {
        for f in &fields.named {
            let name = f.ident.clone();
            let ty = f.ty.clone();



            // add the function argument
            let fn_arg: FnArg = syn::parse_quote! { #name: #ty };
            new_fn.sig.inputs.push(fn_arg);

            if has_attr(f,"variable") {
                let field_name = name.clone().unwrap().to_string();
                let validation: syn::Stmt = syn::parse_quote! {
                    if !matches!(dest, Operand::Variable(_, _)) {
                        panic!("{} must be a variable", #field_name);
                    }
                };
                validation_stmt.push(validation);
            }

            // add the initializer to the struct
            let value: FieldValue = syn::parse_quote! { #name };
            self_expr.fields.push(value);
        }
    }

    for stmt in validation_stmt {
        new_fn.block.stmts.push(stmt);
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
    }.into()
}
