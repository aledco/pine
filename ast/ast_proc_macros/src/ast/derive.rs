use proc_macro::TokenStream;
use quote::quote;

/// Derives the `Ast` and `ScopedAst` traits for a struct.
pub(crate) fn derive_ast_node(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    let name = item_struct.ident.clone();
    quote! {
        impl Ast for #name {
            fn span(&self) -> Span {
                self.span.clone()
            }
        }

        impl ScopedAst for #name {
            fn scope(&self) -> ScopeRef {
                self.scope.clone()
            }

            fn set_scope(&mut self, scope: ScopeRef) {
                self.scope = scope;
            }
        }
    }
        .into()
}

/// Derives the `new` function for a struct.
pub fn derive_new_ast(input: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(input as syn::ItemStruct);
    create_new_fn(&item_struct)
}

/// Creates the `new` function for a struct.
fn create_new_fn(item_struct: &syn::ItemStruct) -> TokenStream {
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

            if let Some(attr) = get_default_attr(f) {
                let mut default_constructor = None::<syn::ExprCall>;
                attr.parse_nested_meta(|meta| {
                    let path = meta.path.clone();
                    default_constructor = Some(syn::parse_quote! { #path() });
                    return Ok(());
                })
                    .unwrap();

                if let Some(default_constructor) = default_constructor {
                    let value: syn::FieldValue = syn::parse_quote! { #name: #default_constructor };
                    self_expr.fields.push(value);
                } else {
                    panic!("invalid default constructor");
                }
            } else {
                // add the function argument
                let fn_arg: syn::FnArg = syn::parse_quote! { #name: #ty };
                new_fn.sig.inputs.push(fn_arg);

                // add the initializer to the struct
                let value: syn::FieldValue = syn::parse_quote! { #name };
                self_expr.fields.push(value);
            }
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

/// Gets the default attribute for a field.
fn get_default_attr(field: &syn::Field) -> Option<&syn::Attribute> {
    for attr in &field.attrs {
        if attr.path().is_ident("default") {
            return Some(attr);
        }
    }

    None
}
