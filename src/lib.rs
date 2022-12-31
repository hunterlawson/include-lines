use proc_macro::TokenStream;
use quote::quote;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use syn::{parse_macro_input, LitStr};

/// Reads in the lines of a file. Given the path to the file.
///
/// Returns type `[&str]`.
#[proc_macro]
pub fn include_lines(input: TokenStream) -> TokenStream {
    let lines = get_lines(parse_macro_input!(input as LitStr).value());

    TokenStream::from(quote! {
        [#(#lines),*]
    })
}

/// Reads in the lines of a file. Given the path to the file.
///
/// Returns type `[String]`
#[proc_macro]
pub fn include_lines_s(input: TokenStream) -> TokenStream {
    let lines = get_lines(parse_macro_input!(input as LitStr).value());

    TokenStream::from(quote! {
        [#(String::from(#lines)),*]
    })
}

/// Counts the number of lines in a file. Given the path to the file.
///
/// Returns type `usize`.
#[proc_macro]
pub fn count_lines(input: TokenStream) -> TokenStream {
    let num_lines = get_lines(parse_macro_input!(input as LitStr).value()).count();

    TokenStream::from(quote! {
        #num_lines
    })
}

// Get an iterator over the lines in a file
fn get_lines(filename: String) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("Error opening the file: ");
    BufReader::new(file).lines().filter_map(|l| l.ok())
}
