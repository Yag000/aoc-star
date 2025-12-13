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
    let year = args.year;

    let expanded = quote! {
        #input_fn

        inventory::submit! {
            aoc_runner::AocEntry {
                day: #day,
                part: #part,
                year: #year,
                func: #fn_name,
            }
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn generate_main(_: TokenStream) -> TokenStream {
    let expanded = quote! {
        fn main() {
    use clap::Parser;

    let args = aoc_star::CommandArgument::parse();

    if args.all {
        for entry in inventory::iter::<aoc_star_runner::AocEntry> {
            println!("Running day {} part {}", entry.day, entry.part);
            (entry.func)(args.clone());
        }
        return;
    }

    let Some(day) = args.day else {
        eprintln!("--day is required unless --all is used");
        return;
    };

    let Some(part) = args.part else {
        eprintln!("part argument is required unless --all is used");
        return;
    };

    let entry = inventory::iter::<aoc_star_runner::AocEntry>
        .find(|e| e.day == day && e.part == part && args.year.map_or(true, |y| e.year == Some(y)));

    match entry {
        Some(e) => (e.func)(args),
        None => eprintln!("No solution found for day {} part {}", day, part),
    }
        }
    };

    expanded.into()
}
