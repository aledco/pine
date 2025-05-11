extern crate proc_macro;
use proc_macro2::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Attribute, Expr, ExprReturn, ExprStruct, Field, FieldValue, FnArg, ItemFn, ItemStruct, LitStr};

#[proc_macro_attribute]
pub fn inst(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    // parse args
    let mut attrs = InstAttributes::default();
    let inst_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with inst_parser);

    let inst_name = attrs.inst_name.unwrap().value();
    if inst_name.is_empty() {
        panic!("Instruction name cannot be empty");
    }

    let operands: syn::ExprArray = attrs.operands.unwrap();

    let helper_attr: Attribute = syn::parse_quote! { #[inst_helper(name = #inst_name, operands = #operands)] };
    item_struct.attrs.insert(0, helper_attr);
    let derive_attr: Attribute = syn::parse_quote! { #[derive(Inst, Debug)] };
    item_struct.attrs.insert(0, derive_attr);
    let new_derive_attr: Attribute = syn::parse_quote! { #[derive(NewInst)] };
    item_struct.attrs.push(new_derive_attr);
    quote! {
        #item_struct
    }.into()
}

// TODO move below to seperate module?
#[derive(Default)]
struct InstAttributes {
    pub inst_name: Option<LitStr>,
    pub operands: Option<syn::ExprArray>
}

impl InstAttributes {
    fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> Result<(), syn::Error> {
        if meta.path.is_ident("name") {
            self.inst_name = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("operands") {
            self.operands = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("Unrecognized inst argument"))
        }
    }
}

#[proc_macro_attribute]
pub fn bin_op(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    let mut attrs = InstAttributes::default();
    if let Some(inst_attr) = get_attr(&item_struct.attrs, "inst_helper") {
        inst_attr.parse_nested_meta(|meta| attrs.parse(meta)).unwrap();
    } else {
        panic!()
    }

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields
            .named
            .push(
            Field::parse_named.parse2(quote! { dest: Operand }).unwrap());

        fields
            .named
            .push(Field::parse_named.parse2(quote! { src1: Operand }).unwrap());

        fields
            .named
            .push(Field::parse_named.parse2(quote! { src2: Operand }).unwrap());
    }

    let mut attrs = BinOpAttributes::default();
    let bin_op_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with bin_op_parser);

    let op = attrs.operator.unwrap();
    let ty1 = attrs.val1_ty.unwrap_or_else(|| syn::parse_quote!(u64));
    let ty2 = attrs.val2_ty.unwrap_or_else(|| syn::parse_quote!(u64));

    let helper_attr: Attribute = syn::parse_quote! { #[bin_op_helper(op = #op, ty1 = #ty1, ty2 = #ty2)] };
    item_struct.attrs.insert(0, helper_attr);
    let derive_attr: Attribute = syn::parse_quote! { #[derive(BinOpInst)] };
    item_struct.attrs.insert(0, derive_attr);

    quote! {
        #item_struct
    }
    .into()
}

// TODO move below to seperate module?
#[derive(Default)]
struct BinOpAttributes {
    pub operator: Option<syn::Ident>,
    pub val1_ty: Option<syn::Type>,
    pub val2_ty: Option<syn::Type>
}

impl BinOpAttributes {
    fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> Result<(), syn::Error> {
        if meta.path.is_ident("op") {
            self.operator = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("ty1") {
            self.val1_ty = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("ty2") {
            self.val2_ty = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("Unrecognized bin_op argument"))
        }
    }
}

#[proc_macro_attribute]
pub fn print(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    let mut attrs = InstAttributes::default();
    if let Some(inst_attr) = get_attr(&item_struct.attrs, "inst_helper") {
        inst_attr.parse_nested_meta(|meta| attrs.parse(meta)).unwrap();
    } else {
        panic!()
    }

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields
            .named
            .push(
                Field::parse_named.parse2(quote! { src: Operand }).unwrap());
    }

    let ty: syn::Type = syn::parse_macro_input!(args);

    let helper_attr: Attribute = syn::parse_quote! { #[print_helper(#ty)] };
    item_struct.attrs.insert(0, helper_attr);
    let derive_attr: Attribute = syn::parse_quote! { #[derive(PrintInst)] };
    item_struct.attrs.insert(0, derive_attr);

    quote! {
        #item_struct
    }.into()
}


#[proc_macro_derive(Inst, attributes(inst_helper))]
pub fn derive_inst(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &item_struct.ident.clone();
    if let Some(inst_attr) = get_attr(&item_struct.attrs, "inst_helper") {
        let mut attrs = InstAttributes::default();
        inst_attr.parse_nested_meta(|meta| attrs.parse(meta)).unwrap();
        let inst_name = attrs.inst_name.unwrap().value();
        let operands = attrs.operands.unwrap();
        let n_operands: usize = operands.elems.len();
        let parse_impl = create_parse_impl(item_struct, n_operands);
        return quote! {
            impl #struct_name {
                pub const NAME: &'static str = #inst_name;
                pub const OPERAND_FORMATS: [OperandFormat;#n_operands] = #operands;
                pub const N_OPERANDS: usize = #n_operands;
            }

            #parse_impl
        }.into();
    }

    panic!("Deriving Inst requires inst helper attribute");
}

fn create_parse_impl(item_struct: ItemStruct, n_operands: usize) -> syn::ItemImpl {
    let struct_name = &item_struct.ident.clone();

    let mut inst_creation_block: syn::Block = syn::parse_quote!({});
    let mut new_expr: syn::ExprCall = syn::parse_quote! { Self::new() };
    for i in 0..n_operands {
        let operand_name: syn::Ident = syn::Ident::new(format!("o{}", i).as_str(), Span::call_site());
        let operand_set_stmt = syn::parse_quote! { let #operand_name = operands.remove(0); };
        inst_creation_block.stmts.push(operand_set_stmt);
        new_expr.args.push(syn::parse_quote! { #operand_name });

    }

    let return_stmt: syn::Stmt = syn::parse_quote! { return Ok(Box::new(#new_expr)); };
    inst_creation_block.stmts.push(return_stmt);

    syn::parse_quote! {
        impl Parse for #struct_name {
            fn parse(line: &Line) -> Result<Box<dyn Instruction>, String> {
                if line.inst_token != Token::Identifier(String::from(Self::NAME)) {
                    return Err(format!("Error at line {}: Cannot parse instruction", line.line));
                }

                if line.operand_tokens.len() != Self::N_OPERANDS {
                    return Err(format!(
                        "Error at line {}: Invalid number of operands for {}. Expected {} but got {}",
                        line.line,
                        Self::NAME,
                        Self::N_OPERANDS,
                        line.operand_tokens.len()
                    ));
                }

                let mut operands: Vec<Operand> = Vec::new();
                for (operand_token, operand_format) in line.operand_tokens.iter().zip(Self::OPERAND_FORMATS.iter()) {
                    let operand = match operand_token {
                        Token::Identifier(v) => {
                            match operand_format {
                                OperandFormat::Variable | OperandFormat::Value  => Ok(Operand::Variable(v.clone())),
                                OperandFormat::Label => Ok(Operand::Label(v.clone())),
                                _=> Err(format!("Error at line {}: Invalid operand format", line.line)),
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
                                _ => Err(format!("Error at line {}: Invalid operand format", line.line))
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


#[proc_macro_derive(NewInst)]
pub fn derive_new_inst(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
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

            // add the function argument
            let fn_arg: FnArg = syn::parse_quote! { #name: #ty };
            new_fn.sig.inputs.push(fn_arg);

            // add the initializer to the struct
            let value: FieldValue = syn::parse_quote! { #name };
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

#[proc_macro_derive(BinOpInst, attributes(bin_op_helper))]
pub fn derive_bin_op_inst(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &item_struct.ident.clone();
    
    if let Some(bin_op_attr) = get_attr(&item_struct.attrs, "bin_op_helper") {
        let mut attrs = BinOpAttributes::default();
        bin_op_attr.parse_nested_meta(|meta| attrs.parse(meta)).unwrap();
        let operator = attrs.operator.unwrap();
        let val1_ty = attrs.val1_ty.unwrap();
        let val2_ty = attrs.val2_ty.unwrap();

        return quote! {
            impl Instruction for #struct_name {
                fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
                    let val1 = crate::cast::from_u64!(self.src1.value(env)?; #val1_ty);
                    let val2 = crate::cast::from_u64!(self.src2.value(env)?; #val2_ty);
                    let res = crate::cast::to_u64!(val1.#operator(val2));
                    self.dest.set_value(res, env);
                    Ok(())
                }

                fn defined_var(&self) -> Option<Operand> {
                    Some(self.dest.clone())
                }

                fn used_vars(&self) -> Vec<Operand> {
                    let mut vars = vec![];
                    if let Operand::Variable(_) = self.src1 {
                        vars.push(self.src1.clone());
                    }

                    if let Operand::Variable(_) = self.src2 {
                        vars.push(self.src2.clone());
                    }

                    vars
                }

                fn validate(&self) -> Result<(), String> {
                    if !matches!(self.dest, Operand::Variable(_)) {
                        Err("dest must be a variable".to_string())
                    } else if matches!(self.src1, Operand::Label(_)) {
                        Err("src1 must be a variable or constant".to_string())
                    } else if matches!(self.src2, Operand::Label(_)) {
                        Err("src2 must be a variable or constant".to_string())
                    }else {
                        Ok(())
                    }
                }
            }

            impl std::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} {} {} {}", Self::NAME, self.dest, self.src1, self.src2)
                }
            }
        }.into();
    }

    panic!("bin_helper attribute is required");
}

#[proc_macro_derive(PrintInst, attributes(print_helper))]
pub fn derive_print_inst(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &item_struct.ident.clone();

    if let Some(print_attr) = get_attr(&item_struct.attrs, "print_helper") {

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

                fn used_vars(&self) -> Vec<Operand> {
                    if let Operand::Variable(_) = self.src {
                        return vec![self.src.clone()];
                    }

                    vec![]
                }

                fn validate(&self) -> Result<(), String> {
                    if matches!(self.src, Operand::Label(_)) {
                        Err("src must be a variable or constant".to_string())
                    } else {
                        Ok(())
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

    unimplemented!()
}

fn get_attr(attrs: &Vec<Attribute>, attr_name: &str) -> Option<Attribute> {
    for attr in attrs {
        if attr.path().is_ident(attr_name) {
            return Some(attr.clone());
        }
    }

    None
}
