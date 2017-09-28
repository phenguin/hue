#![feature(proc_macro)]

extern crate proc_macro;
extern crate case;
extern crate syn;
#[macro_use] extern crate quote;
#[macro_use] extern crate synom;

use std::collections::HashSet;
use proc_macro::TokenStream;
use syn::derive::parsing::derive_input;
use syn::parse::IResult;
use syn::{DeriveInput, Ident, Field, Body, Variant, VariantData, Ty};
use syn::Body::*;
use syn::ident::parsing::ident;
use case::CaseExt;

// TODO Make this more robust.  Just gets last path component now.
fn type_ident(ty: &Ty) -> Option<Ident> {
    use syn::Ty;
    match ty {
        &Ty::Path(_, syn::Path {ref segments, .. }) => {
            segments.into_iter().last().map(|x| x.ident.clone())
        },
        _ => None,
    }
}

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

fn field_expr(types: &HashSet<Ident>, id: &Ident, ty: &Ty) -> quote::Tokens {
    type_ident(ty).map_or(quote!(), |type_id| {
        if types.contains(&type_id) {
            let f = fold_fn_name(&type_id);
            quote!(
                self.#f(&mut it.#id);
            )
        } else {
            quote!()
        }
    })
}


#[proc_macro]
pub fn foldable(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let parsed: FoldableInput = parse_foldable_input(&s).expect("Couldn't parse macro input");
    let type_defs = parsed.defs;

    let all_types = type_defs.iter().cloned().map(|d| d.ident).collect::<HashSet<_>>();

    let trait_name = folder_trait_name(&parsed.name);

    let field_adjustors = |var_data| {
        use VariantData::*;
        println!("{:?}", var_data);
        let pairs: Vec<(Ident,Ty)> = match var_data {
            Unit => Vec::new(), 
            Tuple(fields) => (0..fields.len()).
                map(|i| Ident::from(i.to_string())).
                zip(fields.into_iter().map(|f| f.ty)).
                collect() ,
            Struct(fields) => fields.into_iter().
                map(|f| (f.ident.expect("Struct fields had no name."), f.ty)).
                collect(),
        };
        pairs.into_iter().map(|(id, ty)| {
            field_expr(&all_types, &id, &ty)
        }).collect::<Vec<_>>()
    };

    let fold_functions = type_defs.iter().cloned().map(|def| {
        let func_name = fold_fn_name(&def.ident);
        let ty = &def.ident;

        let body = match def.body {
            Struct(vd) => {
                let stmts = field_adjustors(vd);
                quote! {#(
                    #stmts
                )*}
            },
            Enum(_vs) => {
                // quote! {
                //     #vs
                // }
                unreachable!()
            },
        };
        println!("{}", body);
        quote! {
            fn #func_name(&mut self, it: &mut #ty){
                #body
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
    gen.parse().expect("quote lib generated some garbage.")
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
