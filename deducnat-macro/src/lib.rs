extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, Ident, LitStr, ExprAssign, Token, Expr, Lit, Variant};
use syn::punctuated::Punctuated;


// 2 am witchcraft
#[proc_macro_derive(ReplDoc, attributes(cmd))]
pub fn derive_repl_doc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_name = input.ident;

    let mut docs: HashMap<Ident, Vec<(Variant, LitStr)>> = HashMap::new();

    // Query each doc argument
    for variant in &input.variants {
        // Query the 'cmd' attribute, if existing
        let attr = variant.attrs.iter()
            .find(|a| a.meta.path().is_ident("cmd"));

        match attr {
            None => continue,

            Some(doc_attr) =>  {
                // Parse args of the doc attribute.
                let parser = Punctuated::<ExprAssign, Token![,]>::parse_terminated;
                let doc_args = doc_attr.parse_args_with(parser).unwrap();

                for arg in doc_args {
                    let doc_key = if let Expr::Path(e) = arg.left.as_ref() {
                        e.path.get_ident().unwrap().clone()
                    } else { panic!() };

                    let value = if let Expr::Lit(el) = arg.right.as_ref() {
                        if let Lit::Str(l) = &el.lit {
                            l.clone()
                        } else { panic!() }
                    } else { panic!() };



                    match docs.get_mut(&doc_key) {
                        None => {
                            docs.insert(doc_key, vec![(variant.clone(), value)]);
                        }
                        Some(v) => {
                            v.push((variant.clone(), value))
                        }
                    }
                }
            }
        }
    }



    let function_signs: Vec<_> = docs.iter().map(|(doc_type, _)| {
        quote! {
            fn #doc_type(&self) -> Option<String>
        }
    }).collect();

    let trait_def = quote! {
        trait ReplDoc {
            #(#function_signs);*;
        }
    };

    // Derive functions that will return those doc arguments
    let functions: Vec<_> = docs.iter().map(|(doc_type, values)| {
        let function_sign = quote! {
            fn #doc_type(&self) -> Option<String>
        };

        let arms: Vec<_> = values.iter()
            .map(|(variant, val)| (no_arg_pattern(variant), val))
            .map(|(variant, val)| {
                quote! {
                    #enum_name::#variant => Some(#val.to_string())
                }
            })
            .collect();


        quote! {
            #function_sign {
                match self {
                    #(#arms),*,
                    _ => None
                }
            }
        }
    }).collect();



    let result = quote! {
        #trait_def

        impl ReplDoc for #enum_name {
            #(#functions)*
        }
    };

    result.into()
}






fn no_arg_pattern(variant: &Variant) -> proc_macro2::TokenStream {
    let name = &variant.ident;
    if variant.fields.len() == 0 {
        quote! { #name }
    }
    else {
        let underscores = [Token![_](name.span())].repeat(variant.fields.len());
        quote! { #name ( #(#underscores),* ) }
    }
}