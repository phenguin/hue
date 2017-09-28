#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
#[macro_use] extern crate synom;

use std::collections::HashSet;
use proc_macro::TokenStream;
use syn::derive::parsing::derive_input;
use syn::parse::IResult;
use syn::{DeriveInput, Ident, VariantData, Ty};
use syn::Body::*;
use syn::ident::parsing::ident;

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
    format!("fold_{}", base).into()
}

fn mapper_fn_name(base: &Ident) -> Ident {
    format!("mut_map_{}", base).into()
}

fn mut_fold_fn_name(base:&Ident) -> Ident {
    format!("mut_{}", fold_fn_name(base)).into()
}

struct FoldableInput {
    name: Ident,
    defs: Vec<DeriveInput>,
}

fn field_expr(types: &HashSet<Ident>, id: &Ident, ty: &Ty) -> quote::Tokens {
    type_ident(ty).map_or(quote!(), |type_id| {
        if types.contains(&type_id) {
            let f = mut_fold_fn_name(&type_id);
            quote!(
                self.#f(&mut it.#id);
            )
        } else {
            quote!()
        }
    })
}


#[proc_macro]
pub fn foldable_types(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let parsed: FoldableInput = parse_foldable_input(&s).expect("Couldn't parse macro input");
    let type_defs = parsed.defs;

    let all_types = type_defs.iter().cloned().map(|d| d.ident).collect::<HashSet<_>>();


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
        let ty = &def.ident;
        let body = match def.body {
            Struct(var_data) => {
                let stmts = field_adjustors(var_data);
                quote! {#(
                    #stmts
                )*}
            },
            Enum(variants) => {
                let match_arms = variants.into_iter().map(|v| {
                    use VariantData::*;
                    enum Shape { S, T, U };
                    let var_name = &v.ident;
                    let shape;
                    let matchers: Vec<_> = match v.data {
                        Unit => {
                            shape = Shape::U;
                            Vec::new()
                        },
                        Tuple(fields) => {
                            shape = Shape::T;
                            fields.into_iter().enumerate().
                                map(|(i, f)| (format!("field_{}", i).into(), f.ty)).
                                collect()
                        },
                        Struct(fields) => {
                            shape = Shape::S;
                            fields.into_iter().map(|f| {
                            (f.ident.unwrap(), f.ty)
                            }).collect()
                        },
                    };
                    let exprs: Vec<_> = matchers.iter().map(|pair| {
                        let id = &pair.0;
                        let ty = &pair.1;
                        match type_ident(ty) {
                            None => quote!(),
                            Some(ref type_ident) => {
                                let f = mut_fold_fn_name(&type_ident);
                                if all_types.contains(type_ident) {
                                    quote!(self.#f(#id);)
                                } else {
                                    quote!()
                                }
                            }
                        }
                    }).collect();

                    let pat_names = matchers.into_iter().map(|(x,_)| quote!( ref mut #x));
                    let pattern = match shape {
                        Shape::U => quote!(),
                        Shape::T => quote!(
                            ( #( #pat_names ),* )
                        ),
                        Shape::S => quote!(
                            { #( #pat_names ),* }
                        ),
                    };

                    quote! {
                        self::#ty::#var_name #pattern => {#(
                            #exprs
                        )*}
                    }
                });

                quote!(
                    match *it {#(
                        #match_arms
                    )*}
                )
            },
        };
        println!("{}", body);
        let func_name = fold_fn_name(&def.ident);
        let func_name_mut = mut_fold_fn_name(&def.ident);
        let mut_mapper_name = mapper_fn_name(&def.ident);
        quote! {
            fn #mut_mapper_name(&mut self, _it: &mut self::#ty){
                // No effect by default.
            }

            #[allow(unused_variables)]
            fn #func_name_mut(&mut self, it: &mut self::#ty){
                self.#mut_mapper_name(it);
                #body
            }

            fn #func_name(&mut self, mut it: self::#ty) -> self::#ty {
                self.#func_name_mut(&mut it);
                it
            }
        }
    }).collect::<Vec<_>>();

    let module_name = parsed.name;
    let gen = quote! {
        pub mod #module_name {
            #![allow(non_snake_case)]
            #(
                #type_defs
            )*

            pub trait Folder {#(
                #fold_functions
            )*}

            pub struct IdentityFolder;
            impl Folder for IdentityFolder {}
        }
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
