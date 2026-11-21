use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Expr, ItemStruct, Token, parse_macro_input, parse_quote, punctuated::Punctuated,
};

#[proc_macro]
pub fn create_user(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(
        input with Punctuated::<Expr, Token![,]>::parse_terminated
    );

    let username = &input[0];
    let age = &input[1];

    let expanded = quote! {
        {
            let username = (#username).to_string();
            let age = #age;

            println!("Creating User...");

            User {
                username,
                age,
                struct_name: "?"
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn log_info(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;

    let mut input_struct = parse_macro_input!(item as ItemStruct);

    let struct_name = &input_struct.ident;

    if let syn::Fields::Named(ref mut fields) = input_struct.fields {
        fields.named.push(parse_quote! {
            pub struct_name: &'static str
        });
    }

    let expanded = quote! {
        #input_struct

        impl #struct_name {
            pub fn log_identity(&self) {
                println!(
                    "Hello! This struct is identified as: {}",
                    self.struct_name
                );
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(LogInfo)]
pub fn log_info_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl loginfo::LogInfo for #name {
            fn log_name(&self) {
                println!("Struct Name: {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}
