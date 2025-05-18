# Simple CLI Args Parser

A basic Rust crate for parsing command-line arguments into a structured format. It handles options (short and long, with values) and allows separating positional arguments using the `"--"` separator.

## Features

* Parses the command name (executable path).
* Handles simple arguments (those not starting with hyphens, or appearing after `--`).
* Supports short options (e.g., `-v`).
* Supports long options (e.g., `--verbose`).
* Parses values associated with long options using the equals sign (e.g., `--output=results.txt` or `--data=val1,val2`). Values are automatically split by commas and trimmed.
* Expands bundled short options (e.g., `-abc` is parsed as three distinct options: `-a`, `-b`, and `-c`).
* Uses the special `"--"` argument to denote that all subsequent arguments are simple positional arguments, regardless of whether they look like options.
* Provides a structured `Command` object containing the parsed results.
* Includes a `Display` implementation for the `Command` struct that provides colored, human-readable output (requires the `colored` crate).

## Getting Started

To use this crate, add it as a dependency in your `Cargo.toml` file.

```toml
[dependencies]
# If you have published your crate to crates.io, use the published version:
# simple-cli-args-parser = "0.1.0" # Replace with the actual version

# If this code is part of your project (e.g., in src/args.rs), you might use:
# (No dependency entry needed in this case, just use `mod args;` in main.rs)

# The 'colored' crate is required for the colored output formatting via the Display trait
colored = "2"
````

## Usage

Here's a simple example demonstrating how to use the `init()` function to parse arguments and print the result:

```rust
// src/main.rs

// --- Option 1: If the parsing code is in a module within your project (e.g., src/args.rs) ---
// mod args;
// use args::{init, cmd_str}; // Import the functions from your local module

// --- Option 2: If you are using this as a separate published crate ---
// use simple_cli_args_parser::{init, cmd_str}; // Replace simple_cli_args_parser with your crate name

use colored::Colorize; // Still needed if you use the colored Display implementation

fn main() {
    // Parse the command line arguments provided to the program
    let command = args::init(); // Use `args::init()` for Option 1, or `init()` for Option 2

    // Print the structured parsed command information.
    // This uses the Display implementation which includes coloring.
    println!("{}", command);

    // You can also get the raw command line string:
    // println!("Raw command string: {}", args::cmd_str()); // Use `args::cmd_str()` or `cmd_str()`
}
```

## Parsed Structure (`Command` and `Option`)

The parsing result is encapsulated in the `Command` struct:

```rust
pub struct Command {
    pub cmd_name: String, // The path/name of the executed program.
    pub opts: Vec<Option>, // Parsed options and simple arguments *before* the "--" separator.
    pub args: Vec<String>, // Simple arguments *only* found *after* the "--" separator.
}
```

The `opts` vector contains `Option` structs, each representing a parsed argument before `--`:

```rust
pub struct Option {
    pub opt_type: OptionType, // Classification: Simple, ShortOpt, or LongOpt.
    pub opt_str: String,      // The flag string (e.g., "-v", "--help", "--data").
    pub opt_values: Vec<String>, // Associated values parsed from "=". Empty otherwise.
}

pub enum OptionType {
    Simple,   // e.g., "file.txt", "-" or anything after "--"
    ShortOpt, // e.g., "-f", "-a" (from -abc)
    LongOpt,  // e.g., "--verbose", "--output" (from --output=...)
}
```

**Key Distinction:** Arguments after `"--"` are *always* added directly to the `command.args` list as raw strings, regardless of whether they look like options. Simple arguments *before* `"--"` are added to the `command.opts` list with `OptionType::Simple`.

## Input and Output Example

Consider running your compiled program (`your_program_name`) with the following arguments:

```bash
cargo run -- -iv file.txt --data=apple,banana --verbose -- positional1 --pos-flag another-arg
```

The `get()` function would parse this command line. The `println!("{}", command);` call using the `Display` trait would produce output similar to this (coloring included):

```text
Command: target/debug/your_program_name
Options:
  1. -i (Type: Short Option): Values: None
  2. -v (Type: Short Option): Values: None
  3. file.txt (Type: Simple): Values: None
  4. --data (Type: Long Option): Values: [apple, banana]
  5. --verbose (Type: Long Option): Values: None
Arguments (-- after):
  1. positional1
  2. --pos-flag
  3. another-arg
```

*Note: The exact path for `Command:` will vary based on how you run the program (e.g., `cargo run`, direct execution).*

## Dependencies

This crate depends on the `colored` crate for its colored output in the `Display` implementation for the `Command` struct. If you do not use `println!("{}", command);` and instead access the struct fields directly, you might be able to remove the `colored` dependency, but it is required for the provided `Display` output.

## Limitations and Alternatives

This crate provides a simple, opinionated approach to parsing common command-line patterns. It is **not** a full-featured argument parsing library.

**Limitations:**

  * **No Type Conversion:** All parsed values (`opt_str`, elements in `opt_values`, elements in `args`) are `String`s. You must perform any necessary type conversions (e.g., parsing strings to integers or booleans) manually based on the argument name.
  * **No Validation:** It does not enforce required arguments, check for valid value formats, or handle mutually exclusive options.
  * **No Automatic Help:** It does not generate help messages based on defined arguments.
  * **Simple Structure:** It provides a basic list structure (`opts`, `args`) rather than mapping arguments to specific struct fields or providing subcommand support directly.

For applications requiring more robust argument parsing, validation, type conversion, automatic help generation, or subcommand support, consider using powerful and popular crates from the Rust ecosystem such as:

  * [`clap`](https://crates.io/crates/clap) (Command Line Argument Parser) - Highly flexible and feature-rich.
  * [`structopt`](https://crates.io/crates/structopt) (built on `clap`, uses derive macros for easier definition) - Often simpler for many use cases.

This `simple-cli-args-parser` crate is suitable for small scripts or learning purposes where a minimal and understandable parsing logic is preferred over a comprehensive framework.

## License

(You should choose a license for your code, such as MIT, Apache-2.0, or GPL, and state it here.)
