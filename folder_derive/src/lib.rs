#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
#[macro_use] extern crate synom;

use proc_macro::TokenStream;
use syn::parse::IResult;
use syn::derive::parsing::derive_inputs;
use syn::DeriveInput;

fn parse_many_derive_inputs(input: &str) -> Result<Vec<DeriveInput>, String> {
    unwrap("derive inputs", derive_inputs, input)
}

#[proc_macro]
pub fn foldable(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl #name {
            pub fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
}

fn unwrap<T>(name: &'static str,
             f: fn(&str) -> IResult<&str, T>,
             input: &str)
             -> Result<T, String> {
    match f(input) {
        IResult::Done(mut rest, t) => {
            rest = synom::space::skip_whitespace(rest);
            if rest.is_empty() {
                Ok(t)
            } else if rest.len() == input.len() {
                // parsed nothing
                Err(format!("failed to parse {}: {:?}", name, rest))
            } else {
                Err(format!("unparsed tokens after {}: {:?}", name, rest))
            }
        }
        IResult::Error => Err(format!("failed to parse {}: {:?}", name, input)),
    }
}
