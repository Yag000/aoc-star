mod args;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

use crate::args::AocArgs;

#[proc_macro_attribute]
pub fn star(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AocArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let day = args.day;
    let part = args.part;
    let year = match args.year {
        Some(y) => quote! { Some(#y) },
        None => quote! { None },
    };

    let expanded = quote! {
        #input_fn

        inventory::submit! {
            aoc_star_runner::AocEntry {
                day: #day,
                part: #part,
                year: #year,
                func: #fn_name,
            }
        }
    };

    expanded.into()
}
