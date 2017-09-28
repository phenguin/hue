#![feature(proc_macro)]

extern crate proc_macro;
extern crate case;
extern crate syn;
#[macro_use] extern crate quote;
#[macro_use] extern crate synom;

use proc_macro::TokenStream;
use syn::derive::parsing::derive_input;
use syn::parse::IResult;
use syn::{DeriveInput, Ident};
use syn::ident::parsing::ident;
use case::CaseExt;

named!(foldable_input -> FoldableInput, do_parse!(
    name: ident >>
    punct!(",") >>
    defs: many0!(derive_input) >>
    (FoldableInput {
        name: name,
        defs: defs,
    })
));

fn parse_foldable_input(input: &str) -> Result<FoldableInput, String> {
    unwrap("derive inputs", foldable_input, input)
}

fn fold_fn_name(base: &Ident) -> Ident {
    format!("fold_{}", base).to_lowercase().into()
}

fn folder_trait_name(folder_name: &Ident) -> Ident {
    format!("{}Folder", folder_name.as_ref().to_capitalized()).into()
}


struct FoldableInput {
    name: Ident,
    defs: Vec<DeriveInput>,
}



#[proc_macro]
pub fn foldable(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let parsed: FoldableInput = parse_foldable_input(&s).unwrap();
    let type_defs = parsed.defs;

    let trait_name = folder_trait_name(&parsed.name);
    // Build the impl
    let fold_functions = type_defs.iter().map(|def| {
        let func_name = fold_fn_name(&def.ident);
        let ty = &def.ident;
        quote! {
            fn #func_name(&mut self, it: #ty) {
                println!("Hello from {}", stringify!(#func_name));
            }
        }
    }).collect::<Vec<_>>();
    let gen = quote! {
        #(
            #type_defs
        )*

        trait #trait_name {#(
            #fold_functions
        )*}

        struct DefaultFolder;
        impl #trait_name for DefaultFolder {}

    };

    println!("{}", gen.as_ref());
    gen.parse().unwrap()
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
