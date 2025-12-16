//! # aoc-star-derive
//!
//! Procedural macro to define Advent of Code solution functions. This crate
//! should never be used directly, but rather through the `aoc-star` crate.
//!
//!
//! The `star` macro allows you to annotate functions as solutions for specific days and parts of Advent of Code challenges.
//!
//! ## Example
//! ```ignore
//! use aoc_star_derive::star;
//!
//! #[star(day = 1, part = 1, year = 2024)]
//! fn solve_day1_part1(input: String) -> String {
//!   // solution code here
//!   "solution".to_string()
//!   }
//!
//! ```

mod args;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

use crate::args::AocArgs;

/// Attribute macro to mark a function as an Advent of Code solution
/// # Example
/// ```ignore
/// use aoc_star_derive::star;
///
/// #[star(day = 1, part = 1, year = 2024)]
/// fn solve_day1_part1(input: String) -> String {
///    // solution code here
///    "solution".to_string()
///    }
///
/// #[star(day = 1, part = 2, year = 2024)]
/// fn solve_day1_part2(input: String) -> String {
///   // solution code here
///   "solution".to_string()
///   }
///
/// #[star(day = 2, part = 1)]
/// fn solve_day2_part1(input: String) -> String {
///  // solution code here
///  "solution".to_string()
///  }
/// ```
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

        aoc_star::inventory::submit! {
            aoc_star::AocEntry {
                day: #day,
                part: #part,
                year: #year,
                func: #fn_name,
            }
        }
    };

    expanded.into()
}
