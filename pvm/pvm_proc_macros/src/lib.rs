extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Attribute, Expr, ExprReturn, ExprStruct, Field, FieldValue, FnArg, ItemFn, ItemStruct, LitInt, LitStr};

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
pub fn arithmetic(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    let mut attrs = InstAttributes::default();
    if let Some(inst_attr) = get_attr(&item_struct.attrs, "inst_helper") {
        inst_attr.parse_nested_meta(|meta| attrs.parse(meta)).unwrap();

    } else {
        panic!()
    }

    let n_operands: usize = attrs.operands.unwrap().elems.len();

    if n_operands < 2 || n_operands > 3 {
        panic!("Arithmetic instruction can only have 2 or 3 operands");
    }

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            Field::parse_named
                .parse2(quote! { #[variable] dest: Operand })
                .unwrap(),
        );

        if n_operands == 2 {
            fields
                .named
                .push(Field::parse_named.parse2(quote! { src: Operand }).unwrap());
        } else if n_operands == 3 {
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
            syn::parse_quote! { #[derive(NewArithmetic, ArithmeticInstruction)] }
        } else {
            syn::parse_quote! { #[derive(NewArithmetic)] }
        };
    item_struct.attrs.insert(0, derive_attr);

    quote! {
        #item_struct
    }
    .into()
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
        return quote! {
            impl #struct_name {
                pub const NAME: &'static str = #inst_name;
                pub const OPERAND_FORMATS: [OperandFormat;#n_operands] = #operands;
                pub const N_OPERANDS: usize = #n_operands;
            }

            impl Display for #struct_name {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} {}, {}, {}", #inst_name, self.dest, self.src1, self.src2)
                }
            }
        }.into();
    }

    panic!("Deriving Inst requires inst helper attribute");
}

// fn create_parse_impl(item_struct: ItemStruct, inst_attrs: &InstAttributes) -> TokenStream {
//
// }


#[proc_macro_derive(NewArithmetic, attributes(constant, variable, label))]
pub fn derive_new_arithmetic(input: TokenStream) -> TokenStream {
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
                    if !matches!(dest, Operand::Variable(_)) {
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
pub fn derive_arithmetic_instruction(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &item_struct.ident.clone();
    
    if let Some(bin_op_attr) = get_attr(&item_struct.attrs, "bin_op") {
        let mut operator = None::<syn::Ident>;
        let mut val1_ty: syn::Type = syn::parse_quote!(u64);
        let mut val2_ty: syn::Type = syn::parse_quote!(u64);
        bin_op_attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("op") {
                operator = meta.value()?.parse()?;
            } else if meta.path.is_ident("ty1") {
                val1_ty = meta.value()?.parse()?;
            } else if meta.path.is_ident("ty2") {
                val2_ty = meta.value()?.parse()?;
            }

            Ok(())
        }).unwrap();
        
        let operator = operator.unwrap();
        return quote! {
            impl Instruction for #struct_name {
                fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
                    let val1 = self.src1.value(env)? as #val1_ty;
                    let val2 = self.src2.value(env)? as #val2_ty;
                    self.dest.set_value(val1.#operator(val2) as u64, env);
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
            }
        }.into();
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
