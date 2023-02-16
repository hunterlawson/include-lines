
// /// Include lines from a file and save them to a static variable with the given name.
// /// Each line is saved as type `&str`.
// /// 
// /// Example:
// /// ```
// /// static_include_lines!(MY_LINES, "file.txt");
// /// ```
// /// 
// /// Expands to:
// /// ```
// /// static MY_LINES: [&str; include_lines::count_lines!("file.txt")] = include_lines::include_lines!("file.txt");
// /// ```
// #[allow(unused_macros)]
// macro_rules! static_include_lines {
//     ($var: ident, $filename: literal) => {
//         static $var: [&str; include_lines::count_lines!($filename)] = include_lines::include_lines!($filename);
//     };
// }

// /// This macro functions identically to [static_include_lines], but each line is returned as type `String`.
// #[allow(unused_macros)]
// macro_rules! static_include_lines {
//     ($var: ident, $filename: literal) => {
//         static $var: [String; include_lines::count_lines!($filename)] = include_lines::include_lines_s!($filename);
//     };
// }