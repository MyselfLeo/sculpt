extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::{BTreeMap, HashMap};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemEnum, Ident, LitStr, ExprAssign, Token, Expr, Lit, Variant};
use syn::punctuated::Punctuated;
use syn::token::Token;

#[proc_macro_derive(ReplDoc, attributes(cmd))]
pub fn derive_repl_doc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_name = input.ident;

    let mut command_names: Vec<(Variant, LitStr)> = Vec::new();

    for variant in &input.variants {
        // Query the 'doc' attribute, if existing
        let attr = variant.attrs.iter()
            .find(|a| a.meta.path().is_ident("cmd"));

        match attr {
            None => continue,

            Some(doc_attr) =>  {
                // Parse args of the doc attribute.
                let parser = Punctuated::<ExprAssign, Token![,]>::parse_terminated;
                let doc_args = doc_attr.parse_args_with(parser).unwrap();

                for arg in doc_args {
                    match arg.left.as_ref() {
                        Expr::Path(name) => {
                            let param = name.path.get_ident().unwrap();

                            match param.to_string().as_str() {
                                "name" => {
                                    match arg.right.as_ref() {
                                        Expr::Lit(e) => {
                                            match &e.lit {
                                                Lit::Str(value) => {
                                                    command_names.push((variant.clone(), value.clone()));
                                                },
                                                _ => panic!("Nuhu")
                                            }
                                        }
                                        _ => panic!("Nuhu2")
                                    }
                                },



                                "desc" => todo!(),



                                "schema" => todo!(),




                                _ => panic!()
                            }
                        }
                        _ => panic!()
                    }
                }
            }
        }

        let _x: Vec<String> = command_names.iter().map(|(_, v)| v.value()).collect();
    }


    let name_arms: Vec<_> = command_names.iter()
        .map(|(v, l)| (v.to_token_stream(), l))
        .map(|(v, l)| {
            quote! {
               #v => #l
            }
        })
        .collect();


    let nb =  command_names.len();


    let name_impl = quote! {
        impl ReplDoc for #enum_name {
            fn name(&self) -> String {
                match self {
                    #(#name_arms)*,
                    _ => "#nb"
                }.to_string()
            }
        }
    };

    name_impl.into()
}