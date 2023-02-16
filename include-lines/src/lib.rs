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
//! You can use the `static_include_lines!` and `static_include_lines_s!` macros to initialize static text arrays at compile time:
//!
//! ```ignore
//! use include_lines::{static_include_lines};
//! static_include_lines!(LINES, "file.txt");
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

pub use include_lines_proc::*;

/// Include lines from a file and save them to a static variable with the given name.
/// Each line is saved as type `&str`.
///
/// Example:
/// ```ignore
/// static_include_lines!(MY_LINES, "file.txt");
/// ```
///
/// Expands to:
/// ```ignore
/// static MY_LINES: [&str; include_lines::count_lines!("file.txt")] = include_lines::include_lines!("file.txt");
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! static_include_lines {
    ($var: ident, $filename: literal) => {
        static $var: [&str; include_lines::count_lines!($filename)] =
            include_lines::include_lines!($filename);
    };
}

/// This macro functions identically to [static_include_lines], but each line is returned as type `String`.
#[allow(unused_macros)]
#[macro_export]
macro_rules! static_include_lines_s {
    ($var: ident, $filename: literal) => {
        static $var: [String; include_lines::count_lines!($filename)] =
            include_lines::include_lines_s!($filename);
    };
}
