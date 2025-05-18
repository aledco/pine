use crate::common;
use crate::inst::InstAttributes;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::StmtMacro;

/// Implements the `Inst` derive macro.
pub fn derive_inst(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &item_struct.ident.clone();
    if let Some(inst_attr) = common::get_attr(&item_struct.attrs, "inst_helper") {
        let mut attrs = InstAttributes::default();
        inst_attr
            .parse_nested_meta(|meta| attrs.parse(meta))
            .unwrap();
        let inst_name = attrs.inst_name.unwrap().value();
        let operand_formats = attrs.operands.unwrap();
        let n_operands: usize = operand_formats.elems.len();

        let validate_impl = create_validate_impl(&item_struct, &operand_formats);
        let parse_impl = create_parse_impl(&item_struct, n_operands);
        let display_impl = create_display_impl(&item_struct);
        return quote! {
            impl #struct_name {
                pub const NAME: &'static str = #inst_name;
                pub const OPERAND_FORMATS: [OperandFormat;#n_operands] = #operand_formats;
                pub const N_OPERANDS: usize = #n_operands;
            }

            #validate_impl

            #parse_impl

            #display_impl
        }
        .into();
    }

    panic!("Deriving Inst requires inst helper attribute");
}

/// Implements the `NewInst` derive macro.
pub(crate) fn derive_new_inst(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &item_struct.ident.clone();

    let mut new_fn: syn::ItemFn = syn::parse_quote! {
        pub fn new() -> Self {

        }
    };

    let mut self_expr: syn::ExprStruct = syn::parse_quote! { Self {} };
    if let syn::Fields::Named(ref fields) = item_struct.fields {
        for f in &fields.named {
            let name = f.ident.clone();
            let ty = f.ty.clone();

            // add the function argument
            let fn_arg: syn::FnArg = syn::parse_quote! { #name: #ty };
            new_fn.sig.inputs.push(fn_arg);

            // add the initializer to the struct
            let value: syn::FieldValue = syn::parse_quote! { #name };
            self_expr.fields.push(value);
        }
    }

    let return_expr = syn::Expr::Return(syn::ExprReturn {
        attrs: vec![],
        return_token: Default::default(),
        expr: Some(Box::new(syn::Expr::Struct(self_expr))),
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

/// Creates the impl block for the `Validate` trait.
fn create_validate_impl(
    item_struct: &syn::ItemStruct,
    operand_formats: &syn::ExprArray,
) -> syn::ItemImpl {
    let struct_name = &item_struct.ident.clone();

    let operands = get_operands(item_struct);

    let mut validate_fn: syn::ItemFn = syn::parse_quote! {
        fn validate(&self) -> Result<(), crate::error::Error> {
            Ok(())
        }
    };

    for (operand, format) in operands.iter().zip(&operand_formats.elems).rev() {
        let validate_stmt: syn::Stmt = syn::parse_quote! {
            (#format).validate(&self.#operand)?;
        };

        validate_fn.block.stmts.insert(0, validate_stmt);
    }

    syn::parse_quote! {
        impl Validate for #struct_name {
            #validate_fn
        }
    }
}

/// Creates the impl block for the `Parse` trait.
fn create_parse_impl(item_struct: &syn::ItemStruct, n_operands: usize) -> syn::ItemImpl {
    let struct_name = &item_struct.ident.clone();

    let mut inst_creation_block: syn::Block = syn::parse_quote!({});
    let mut new_expr: syn::ExprCall = syn::parse_quote! { Self::new() };
    for i in 0..n_operands {
        let operand_name: syn::Ident =
            syn::Ident::new(format!("o{}", i).as_str(), Span::call_site());
        let operand_set_stmt = syn::parse_quote! { let #operand_name = operands.remove(0); };
        inst_creation_block.stmts.push(operand_set_stmt);
        new_expr.args.push(syn::parse_quote! { #operand_name });
    }

    let return_stmt: syn::Stmt = syn::parse_quote! { return Ok(Box::new(#new_expr)); };
    inst_creation_block.stmts.push(return_stmt);

    syn::parse_quote! {
        impl Parse for #struct_name {
            fn parse(line: &Line) -> Result<Box<dyn Instruction>, crate::error::Error> {
                if line.inst_token != Token::Identifier(String::from(Self::NAME)) {
                    return Err(crate::parse::ParseError::invalid_token(line.line));
                }

                if line.operand_tokens.len() != Self::N_OPERANDS {
                    return Err(crate::parse::ParseError::invalid_number_of_operands(
                        Self::NAME,
                        Self::N_OPERANDS,
                        line.operand_tokens.len(),
                        line.line,
                    ));
                }

                let mut operands: Vec<Operand> = Vec::new();
                for (operand_token, operand_format) in line.operand_tokens.iter().zip(Self::OPERAND_FORMATS.iter()) {
                    let operand = match operand_token {
                        Token::Identifier(v) => {
                            match operand_format {
                                OperandFormat::Variable | OperandFormat::Value  => Ok(Operand::Variable(v.clone())),
                                OperandFormat::Label => Ok(Operand::Label(v.clone())),
                                _=> Err(crate::parse::ParseError::invalid_operand_format(line.line)),
                            }
                        }
                        Token::Literal(l) => {
                            match operand_format {
                                OperandFormat::Constant | OperandFormat::Value => {
                                    match l {
                                        Literal::Integer(i) => Ok(Operand::Constant(crate::cast::to_u64!(*i))),
                                        Literal::Float(f) => Ok(Operand::Constant(crate::cast::to_u64!(*f))),
                                        Literal::Char(c) => Ok(Operand::Constant(crate::cast::to_u64!(*c))),
                                        Literal::String(_) => unimplemented!(),
                                    }
                                }
                                _ => Err(crate::parse::ParseError::invalid_operand_format(line.line))
                            }
                        },
                    };

                    match operand {
                        Ok(operand) => operands.push(operand),
                        Err(e) => return Err(e),
                    }
                }

                #inst_creation_block
            }
        }
    }
}

/// Creates the impl block for the `Display` trait.
fn create_display_impl(item_struct: &syn::ItemStruct) -> syn::ItemImpl {
    let struct_name = &item_struct.ident.clone();

    let operands = get_operands(item_struct);

    let operands_format_str: String = operands
        .iter()
        .map(|o| format!("{{{}}}", o))
        .enumerate()
        .map(|(i, s)| if i == 0 { s } else { format!(" {}", s) })
        .collect();
    let format_str =  if operands_format_str.is_empty() {
        "{}".to_string()
    } else {
        format!("{{}} {}", operands_format_str)
    };
    
    let operand_stmts = operands
        .iter()
        .map(|o| syn::parse_quote! {
            let #o = self.#o.clone();
        })
        .collect::<Vec<syn::Stmt>>();

    let write_macro: syn::ExprMacro = syn::parse_quote! {
        write!(f, #format_str, Self::NAME)
    };

    let mut fmt_fn: syn::ItemFn = syn::parse_quote! {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            #write_macro
        }
    };

    for stmt in operand_stmts.into_iter().rev() {
        fmt_fn.block.stmts.insert(0, stmt);
    }

    syn::parse_quote! {
        impl std::fmt::Display for #struct_name {
            #fmt_fn
        }
    }
}

/// Gets the identifiers of the operands of the instruction.
fn get_operands(item_struct: &syn::ItemStruct) -> Vec<syn::Ident> {
    let mut operands = Vec::new();
    if let syn::Fields::Named(ref fields) = item_struct.fields {
        for f in &fields.named {
            let ty = f.ty.clone();
            if ty.clone().into_token_stream().to_string() == stringify!(Operand) {
                let name = f.ident.clone().unwrap();
                operands.push(name);
            }
        }
    }

    operands
}