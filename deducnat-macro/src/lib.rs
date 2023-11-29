extern crate proc_macro;
use proc_macro::TokenStream;
use std::collections::HashMap;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, Ident, Meta, LitStr};

#[proc_macro_derive(ReplDoc)]
pub fn derive_repl_doc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);

    let mut command_names: HashMap<Ident, LitStr> = HashMap::new();

    for variant in input.variants {
        let ident = &variant.ident;
        let attrs = &variant.attrs;

        let command_attr = attrs
            .iter()
            .find(|e| e.meta.path().is_ident("doc"));

        match command_attr {
            Some(Meta::Path(_)) | Some(Meta::NameValue(_)) => { compile_error!("Syntax: #[doc(name=\"name\", ...]") }
            Some(Meta::List(l)) => {
                l.parse_nested_meta(|attr| {
                   match attr.path.get_ident().map(|ident| ident.to_string().as_str()) {
                       Some("name") => {
                           let value = attr.value()?;
                           let s: LitStr = value.parse()?;
                           command_names.insert(ident.clone(), s);
                       }
                       Some("desc") => {todo!()}
                       Some("schema") => {todo!()}
                       _ => { compile_error!("Syntax: #[doc(name=\"name\", ...]") }
                   };

                    Ok(())
                })?;
            }
            _ => {}
        }

        println!("Names: {:?}", command_names);
    }


    let expanded = quote! {
        #input
    };
}




#[proc_macro_attribute]
pub fn doc(attrs: TokenStream, item: TokenStream) -> TokenStream {
    println!("Parsed doc: {:?}", attrs);
    item
}