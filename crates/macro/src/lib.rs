
extern crate proc_macro;


use proc_macro2::{
    TokenStream,
    Ident,
};

use quote::quote;
use syn::{
    Item,
    ItemFn,
    punctuated::Punctuated,
    Token,
};

#[proc_macro_attribute]
pub fn log_top_and_bottom(_attr: proc_macro::TokenStream, stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse(stream.clone()).unwrap();
    if let Item::Fn(f) = item {
        transform(f).into()
    } else {
        stream
    }
}

fn transform(f: ItemFn) -> TokenStream {
    let vis = f.vis.clone();
    let sig = f.sig.clone();
    let body = f.block.clone();
    let original_str = format!("{}", sig.ident);
    let quoted = quote! {
        #vis#sig {
            log::debug!("start {}", #original_str);
            let ret = (move || {
                #body
            })();
            log::debug!("end {}", #original_str);
            ret
        }
    };
    quoted.into()
}