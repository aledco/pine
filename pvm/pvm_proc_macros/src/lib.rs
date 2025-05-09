extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, parse_macro_input, Attribute, BinOp, Expr, ExprReturn, ExprStruct, Field,
    FieldValue, FnArg, ItemFn, ItemStruct, LitInt,
};

#[proc_macro_attribute]
pub fn arithmetic(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    let operands: LitInt = syn::parse_macro_input!(args as LitInt);
    let operands: usize = operands.base10_parse().unwrap();

    // let meta_parser = syn::meta::parser(|meta| {
    //     if meta.path.is_ident("operands") {
    //         operands = Some(meta.value()?.parse()?);
    //         Ok(())
    //     // } else if meta.path.is_ident("operator") {
    //     //     operator = Some(meta.value()?.parse()?);
    //     //     Ok(())
    //     } else {
    //         Err(meta.error("unsupported arithmetic property"))
    //     }
    // });
    //
    // parse_macro_input!(args with meta_parser);

    if operands < 1 || operands > 2 {
        panic!("Arithmetic instruction can only have 1 or 2 operands");
    }

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            Field::parse_named
                .parse2(quote! { #[variable] dest: Operand })
                .unwrap(),
        );

        if operands == 1 {
            // TODO add way to specify that it cannot be a label too (maybe allowed and not_allowed attributes with args instead)
            fields
                .named
                .push(Field::parse_named.parse2(quote! { src: Operand }).unwrap());
        } else if operands == 2 {
            fields
                .named
                .push(Field::parse_named.parse2(quote! { src1: Operand }).unwrap());

            fields
                .named
                .push(Field::parse_named.parse2(quote! { src2: Operand }).unwrap());
        }
    }

    let derive_attr: Attribute =
        if has_attr(&item_struct.attrs, "bin_op") || has_attr(&item_struct.attrs, "un_op") {
            syn::parse_quote! { #[derive(NewArithmetic, ArithmeticInstruction, Debug)] }
        } else {
            syn::parse_quote! { #[derive(NewArithmetic, Debug)] }
        };
    item_struct.attrs.push(derive_attr);

    quote! {
        #item_struct
    }
    .into()
}

#[proc_macro_derive(NewArithmetic, attributes(constant, variable, label))]
pub fn derive_arithmetic_instruction(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
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

            if has_attr(&f.attrs, "variable") {
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
    }
    .into()
}

#[proc_macro_derive(ArithmeticInstruction, attributes(bin_op))]
pub fn derive_new_ast(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &item_struct.ident.clone();

    if has_attr(&item_struct.attrs, "bin_op") {
        let operator_attr = get_attr(&item_struct.attrs, "bin_op").unwrap();
        let operator: BinOp = operator_attr
            .parse_args()
            .expect("bin_op argument is required");
        
        return quote! {
            impl Instruction for #struct_name {
                fn execute(&mut self, context: &mut Environment) -> Result<(), String> {
                    let val1 = self.src1.value()?;
                    let val2 = self.src2.value()?;
                    self.dest.set_value(val1 #operator val2);
                    Ok(())
                }
            }
        }
        .into();
    }

    unimplemented!()
}

fn has_attr(attrs: &Vec<Attribute>, attr_name: &str) -> bool {
    for attr in attrs {
        if attr.path().is_ident(attr_name) {
            return true;
        }
    }

    false
}

fn get_attr(attrs: &Vec<Attribute>, attr_name: &str) -> Option<Attribute> {
    for attr in attrs {
        if attr.path().is_ident(attr_name) {
            return Some(attr.clone());
        }
    }

    None
}
