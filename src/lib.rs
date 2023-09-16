use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(GetSet)]
pub fn macro_learn_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named_fields) => &named_fields.named,
            _ => return quote! {}.into(),
        },
        _ => return quote! {}.into(),
    };

    let mut field_functions = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let get_function_name = Ident::new(&format!("get_{}", field_name), field_name.span());
        let set_function_name = Ident::new(&format!("set_{}", field_name), field_name.span());

        field_functions.push(quote! {
            pub fn #get_function_name(&self) -> &#field_type {
                &self.#field_name
            }
            pub fn #set_function_name(&mut self, set_value : #field_type) {
                self.#field_name = set_value;
            }
        });
    }

    quote! {
     impl #struct_name {
         #(#field_functions)*
     }
    }
    .into()
}
