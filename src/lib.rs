//! # include-lines
//!
//! Rust macros for reading in all lines from a file at compile time. This can be very useful for loading static data.
//!
//! ## Examples
//!
//! For the examples, there is a file `file.txt` in the same directory as the project's Cargo.toml file:
//!
//! ```text
//! these
//! are
//! file
//! lines
//! ```
//!
//! ### Read in a file and store it an an array of type `[&'static str]`
//!
//! ```ignore
//! use include_lines::include_lines;
//! let lines = include_lines!("file.txt");
//! ```
//!
//! For the example file, this expands to:
//!
//! ```
//! let lines = [
//!     "these",
//!     "are",
//!     "file",
//!     "lines",
//! ];
//! ```
//!
//! ### Read in a file and store it an an array of type `[String]`
//!
//! ```ignore
//! use include_lines::include_lines_s;
//! let lines = include_lines_s!("file.txt");
//! ```
//!
//! For the example file, this expands to:
//!
//! ```
//! let lines = [
//!     String::from("these"),
//!     String::from("are"),
//!     String::from("file"),
//!     String::from("lines"),
//! ];
//! ```
//!
//! ### Get the number of lines in a file at compile time as type `usize`
//!
//! ```ignore
//! use include_lines::count_lines;
//! let num_lines = count_lines!("file.txt");
//! ```
//!
//! For the example file, this expands to:
//!
//! ```
//! let num_lines = 4usize;
//! ```
//!
//! ### Create a static array from a file at compile time
//!
//! You can use the `include_lines!` and `count_lines!` macros to initialize static text arrays at compile time:
//!
//! ```ignore
//! use include_lines::{count_lines, include_lines};
//! static LINES: [&str; count_lines!("file.txt")] = include_lines!("file.txt");
//! ```
//! For the example file, this expands to:
//!
//! ```
//! static LINES: [&str; 4] = [
//!     "these",
//!     "are",
//!     "file",
//!     "lines",
//! ];
//! ```

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
