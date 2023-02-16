# include-lines

Rust macros for reading in all lines from a file at compile time. This can be very useful for loading static data.

## Examples

For the examples, there is a file `file.txt` in the same directory as the project's Cargo.toml file:

```text
these
are
file
lines
```

### Read in a file and store it an an array of type `[&'static str]`

```rust
use include_lines::include_lines;
let lines = include_lines!("file.txt");
```

For the example file, this expands to:

```rust
let lines = [
    "these",
    "are",
    "file",
    "lines",
];
```

### Read in a file and store it an an array of type `[String]`

```rust
use include_lines::include_lines_s;
let lines = include_lines_s!("file.txt");
```

For the example file, this expands to:

```rust
let lines = [
    String::from("these"),
    String::from("are"),
    String::from("file"),
    String::from("lines"),
];
```

### Get the number of lines in a file at compile time as type `usize`

```rust
use include_lines::count_lines;
let num_lines = count_lines!("file.txt");
```

For the example file, this expands to:

```rust
let num_lines = 4usize;
```

### Create a static array from a file at compile time

You can use the `static_include_lines!` and `static_include_lines_s!` macros to initialize static text arrays at compile time:

```rust
use include_lines::{static_include_lines};
static_include_lines!(LINES, "file.txt");
```

For the example file, this expands to:

```rust
static LINES: [&str; 4] = [
    "these",
    "are",
    "file",
    "lines",
];
```
