#![doc(test(attr(ignore)))]
#![doc = include_str!("../../README.md")]

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
