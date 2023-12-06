extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemEnum, Ident, LitStr, ExprAssign, Token, Expr, Lit, Variant};
use syn::punctuated::Punctuated;


#[proc_macro_derive(EnumDoc, attributes(cmd))]
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

    // Nothing to document
    if docs.len() == 0 { return quote! {}.into()}



    let function_signs: Vec<_> = docs.iter().map(|(doc_type, _)| {
        quote! {
            fn #doc_type(&self) -> Option<String>
        }
    }).collect();

    let trait_name = format_ident!("{enum_name}ReplDoc");
    let trait_def = quote! {
        trait #trait_name {
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

        impl #trait_name for #enum_name {
            #(#functions)*
        }
    };

    result.into()
}











#[proc_macro_derive(EnumType)]
pub fn derive_enum_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    let enum_name = input.ident;
    let new_enum_name = format_ident!("{}Type", enum_name);


    // match-pattern (ex: Foo::Bar(_, _)) & variant ident (ex: Bar)
    let type_assoc = input.variants.iter()
        .map(|v| (no_arg_pattern(v), v.ident.clone()))
        .collect::<Vec<_>>();



    let idents = type_assoc.iter()
        .map(|(_, t)| t.clone())
        .collect::<Vec<_>>();

    let enum_def = quote! {
        pub enum #new_enum_name {
            #(#idents),*
        }
    };

    let type_trait_name = format_ident!("{enum_name}Typed");
    let type_trait_def = quote! {
        pub trait #type_trait_name {
            fn get_type(&self) -> #new_enum_name;
        }
    };

    let assocs = type_assoc.iter()
        .map(|(pat, i)| {
            quote! {
                #enum_name::#pat => #new_enum_name::#i
            }
        })
        .collect::<Vec<_>>();

    let get_type_def = quote! {
        fn get_type(&self) -> #new_enum_name {
            match self {
                #(#assocs),*,
            }
        }
    };


    let default_trait_name = format_ident!("{new_enum_name}Default");
    let default_trait_def = quote! {
        pub trait #default_trait_name {
            fn get_default(&self) -> #enum_name;
        }
    };

    let default_assocs = input.variants.iter()
        .map(|v| (v.ident.clone(), default_args(v)))
        .map(|(i, dft)| {
            quote! {
                #new_enum_name::#i => #enum_name::#dft
            }
        })
        .collect::<Vec<_>>();

    let get_default_def = quote! {
        fn get_default(&self) -> #enum_name {
            match self {
                #(#default_assocs),*,
            }
        }
    };


    let enum_impl = quote! {
        impl #type_trait_name for #enum_name {
            #get_type_def
        }
    };

    let type_impl = quote! {
        impl #default_trait_name for #new_enum_name {
            #get_default_def
        }
    };



    let res = quote! {
        #enum_def

        #type_trait_def
        #enum_impl

        #default_trait_def
        #type_impl
    };

    res.into()
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


fn default_args(variant: &Variant) -> proc_macro2::TokenStream {
    let name = &variant.ident;
    if variant.fields.len() == 0 {
        quote! { #name }
    }
    else {
        let defaults = variant.fields.iter()
            .map(|f| {
                f.ty.clone()
            })
            .map(|t| {
               quote! {
                   #t::default()
               }
            })
            .collect::<Vec<_>>();

        quote! {
            #name(#(#defaults),*)
        }
    }
}





