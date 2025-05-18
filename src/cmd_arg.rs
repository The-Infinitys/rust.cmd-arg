//! This module provides a structured way to parse command-line arguments,
//! separating them into a command name, a list of parsed options (including
//! simple arguments before a separator), and a list of simple arguments
//! that appear after a special separator.
//!
//! It supports:
//! - Simple arguments (e.g., `myfile.txt`)
//! - Short options (e.g., `-v`)
//! - Long options (e.g., `--verbose`)
//! - Long options with values (e.g., `--output=results.txt` or `--data=val1,val2`)
//! - Bundled short options (e.g., `-abc` which is parsed as `-a`, `-b`, and `-c`)
//!
//! A special argument, `"--"`, can be used to separate options from
//! positional arguments. Any argument appearing after `"--"` is treated
//! purely as a simple argument and collected into the `args` field,
//! bypassing the standard option parsing logic. Simple arguments
//! appearing *before* `"--"` are included in the `opts` list with
//! `OptionType::Simple`.
//!
//! This module is designed to provide a basic, opinionated parsing structure
//! suitable for simple command-line tools. For more complex needs (like
//! type conversion, required arguments, subcommands, automatic help message
//! generation), consider using more comprehensive crates like `clap` or `structopt`.

use colored::Colorize;
use std::env;
use std::fmt;
/// Represents the classification of a command-line argument based on its format.
/// This helps distinguish between different syntactical types of input.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
    /// Represents a standard positional argument or any argument that does not
    /// conform to the syntax of a short or long option. This includes:
    /// - Strings that do not start with a hyphen (e.g., `"input.txt"`).
    /// - A single hyphen character (`"-"`).
    /// - Any argument that appears *after* the special `"--"` separator.
    ///
    /// Simple arguments appearing *before* the `"--"` separator are stored
    /// in the `opts` vector of the `Command` struct. Simple arguments
    /// appearing *after* `"--"` are stored in the `args` vector.
    Simple,
    /// Represents an argument starting with a single hyphen, followed by one
    /// or more characters.
    /// - A single short option consists of `-` followed by one character (e.g., `"-f"`).
    /// - Multiple short options can be bundled together after a single hyphen
    ///   (e.g., `"-abc"` which is parsed as three distinct `ShortOpt`s: `"-a"`, `"-b"`, `"-c"`).
    ///
    /// Note: A single hyphen `"-"` is **not** classified as `ShortOpt`; it is `Simple`.
    ShortOpt,
    /// Represents an argument starting with two hyphens, followed by one or more
    /// characters (e.g., `"--verbose"`).
    /// - Long options can optionally have a value attached using an equals sign
    ///   (e.g., `"--output=results.txt"`). The value can also be comma-separated
    ///   (e.g., `"--data=val1,val2"`).
    /// - The value part is stored in the `opt_values` field, while the flag
    ///   part (e.g., `"--output"` or `"--data"`) is stored in `opt_str`.
    LongOpt,
}

/// Holds the parsed information for a single command-line argument that was
/// classified as an option or a simple argument occurring before the `"--"` separator.
#[derive(Debug, Clone)]
pub struct Option {
    /// The classification of this argument's format.
    pub opt_type: OptionType,
    /// The string representation of the option or simple argument itself.
    /// - For `ShortOpt` (`-v`), this is the full string like `"-v"`.
    /// - For `LongOpt` without a value (`--help`), this is the full string like `"--help"`.
    /// - For `LongOpt` with a value (`--data=val`), this is only the flag part before the equals sign (e.g., `"--data"`).
    /// - For `Simple` arguments before `"--"` (`myfile.txt`), this is the full string.
    pub opt_str: String,
    /// A list of associated values parsed from the argument string.
    /// - For `LongOpt` with values (e.g., `--data=v1,v2`), this will contain
    ///   the parsed, comma-separated values (`["v1", "v2"]`).
    /// - For all other argument types (`Simple`, `ShortOpt`, `LongOpt` without values,
    ///   and bundled `ShortOpt`s resulting from splitting, like `-a` from `-abc`),
    ///   this vector will typically be empty.
    pub opt_values: Vec<String>,
}

/// Represents the complete structured result of parsing the command line.
/// It separates the program name, the options/initial simple arguments,
/// and any simple arguments that appeared after the `"--"` separator.
#[derive(Debug)]
pub struct Command {
    /// The name of the executable program being run. This is typically the
    /// first argument provided by the environment.
    pub cmd_name: String,
    /// A vector containing all arguments parsed as options (`ShortOpt`, `LongOpt`)
    /// and any `Simple` arguments that appeared *before* the special `"--"` separator.
    /// The order in this vector reflects the order in which they appeared on the command line.
    pub opts: Vec<Option>,
    /// A vector containing all arguments that appeared *after* the special `"--"`
    /// separator. These arguments are always treated as `Simple` and their
    /// content is stored directly here without further parsing for options.
    pub args: Vec<String>,
}

// Implement the Display trait for Command to allow easy printing of the parsed result.
// Coloring is included to enhance readability in console output.
impl fmt::Display for Command {
    /// Formats the content of the `Command` struct into a human-readable string
    /// with console coloring provided by the `colored` crate.
    /// It displays the command name, followed by a list of parsed options
    /// (from the `opts` field) and a list of arguments found after `"--"`
    /// (from the `args` field).
    ///
    /// # Arguments
    ///
    /// * `f`: The formatter to write into.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the formatting was successful.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display command name with bold cyan label and blue value
        writeln!(f, "{}: {}", "Command".cyan().bold(), self.cmd_name.blue())?;

        // Display the list of options (before the -- separator)
        writeln!(f, "{}:", "Options".green().bold())?;
        if self.opts.is_empty() {
            // Indicate if no options were found before --
            writeln!(f, "  {}", "No Options provided (before --).".red())?;
        } else {
            // Iterate through and display each parsed option
            for (i, opt) in self.opts.iter().enumerate() {
                // Determine color and text for the option type
                let opt_type = match opt.opt_type {
                    OptionType::Simple => "Simple".purple(),
                    OptionType::ShortOpt => "Short Option".yellow(),
                    OptionType::LongOpt => "Long Option".cyan(),
                };
                // Format the associated values, indicating "None" if empty
                let values = if opt.opt_values.is_empty() {
                    "None".red().to_string()
                } else {
                    format!("[{}]", opt.opt_values.join(", ").green())
                };
                // Write the formatted option line
                writeln!(
                    f,
                    "  {}. {} ({}: {}): {}: {}",
                    (i + 1).to_string().bold(), // Item number in bold
                    opt.opt_str.magenta(),      // The option string itself in magenta
                    "Type".cyan(),              // Label "Type" in cyan
                    opt_type,        // The determined option type (colored based on type)
                    "Values".cyan(), // Label "Values" in cyan
                    values           // The associated values (colored based on presence)
                )?;
            }
        }

        // Display the list of arguments (after the -- separator)
        writeln!(f, "{}:", "Arguments (-- after)".green().bold())?;
        if self.args.is_empty() {
            // Indicate if no arguments were found after --
            writeln!(f, "  {}", "No arguments provided after --.".red())?;
        } else {
            // Iterate through and display each argument after --
            for (i, arg) in self.args.iter().enumerate() {
                // Write the formatted argument line
                writeln!(f, "  {}. {}", (i + 1).to_string().bold(), arg.blue())?;
            }
        }

        Ok(())
    }
}

impl Command {
    /// Creates a new, empty `Command` instance with a specified command name.
    /// The `opts` and `args` vectors are initialized as empty.
    ///
    /// This is a private helper function used internally by `init`.
    ///
    /// # Arguments
    ///
    /// * `cmd_name`: The string representing the name of the command (e.g., "myprogram").
    ///
    /// # Returns
    ///
    /// A fresh `Command` instance ready to be populated with parsed arguments.
    fn new(cmd_name: String) -> Self {
        Command {
            cmd_name,
            opts: Vec::new(),
            args: Vec::new(), // Initialize the args field as empty
        }
    }

    /// Adds a parsed `Option` struct to the internal list of options (`self.opts`).
    /// This method is used for arguments parsed before the `"--"` separator.
    ///
    /// This is a private helper function used internally by `init`.
    ///
    /// # Arguments
    ///
    /// * `opt`: The `Option` struct containing the parsed details of an argument
    ///   that is considered either a standard option or a simple argument
    ///   appearing before `"--"`.
    fn add_opt(&mut self, opt: Option) {
        self.opts.push(opt);
    }
}

/// Analyzes a single argument string (that appears before the `"--"` separator)
/// and determines its classification as `Simple`, `ShortOpt`, or `LongOpt`
/// based on its starting characters and length.
///
/// This function is used during the parsing process for arguments encountered
/// before the `"--"` separator. Arguments after `"--"` are *not* passed
/// through this function; they are always treated as `Simple` and placed
/// directly into the `args` field.
///
/// # Arguments
///
/// * `arg`: A string slice representing the command-line argument to classify.
///
/// # Returns
///
/// An `OptionType` enum variant indicating how the argument should be
/// interpreted by the parser.
fn determine_opt_type(arg: &str) -> OptionType {
    if arg.starts_with("--") {
        // Starts with double hyphen: Long Option
        OptionType::LongOpt
    } else if arg.starts_with("-") && arg.len() > 1 {
        // Starts with single hyphen and has more characters: Short Option(s)
        // Note: A single '-' is explicitly excluded here and falls to Simple.
        OptionType::ShortOpt
    } else {
        // Does not start with hyphen, or is just a single hyphen: Simple Argument
        OptionType::Simple
    }
}

/// Parses a string expected to contain one or more values separated by commas.
/// It splits the string, trims leading/trailing whitespace from each resulting
/// value, and collects them into a `Vec<String>`. Empty strings resulting
/// from the split (e.g., from trailing commas or `,,`) are filtered out.
///
/// This function is used to process values associated with long options
/// that use the `=` syntax with comma-separated lists (e.g., `--data=v1, v2,v3`).
///
/// # Arguments
///
/// * `value`: The string slice containing the comma-separated values.
///
/// # Returns
///
/// A vector of strings where each string is a trimmed, non-empty value
/// extracted from the input string. Returns an empty vector if the input
/// string is empty or contains only whitespace/commas.
fn parse_values(value: &str) -> Vec<String> {
    value
        .split(',') // Split the string by the comma delimiter
        .map(|s| s.trim().to_string()) // Trim whitespace from each resulting piece and convert to String
        .filter(|s| !s.is_empty()) // Remove any empty strings that resulted from the split/trimming
        .collect() // Collect the valid strings into a vector
}

/// The main function of this module. It retrieves the command-line arguments
/// provided to the program from the environment, parses them according to the
/// module's rules, and returns a structured `Command` object.
///
/// The parsing logic iterates through the arguments:
/// 1. The first argument is treated as the command name.
/// 2. Subsequent arguments are processed sequentially.
/// 3. If a `"--"` argument is encountered, all remaining arguments
///    are immediately collected into the `command.args` field without
///    further interpretation as options. The parsing loop then terminates.
/// 4. Arguments before `"--"` are classified using `determine_opt_type`.
///    - `LongOpt`s are checked for an `=` sign to extract values. The key part
///      (e.g., `--option`) goes into `opt_str`, values (if any) into `opt_values`.
///    - `ShortOpt`s are checked for bundling (length > 2). Bundled characters
///      (e.g., `a`, `b`, `c` from `-abc`) are turned into individual `ShortOpt`s
///      (`-a`, `-b`, `-c`). Single `ShortOpt`s (`-v`) are added directly.
///    - `Simple` arguments before `"--"` are added to the `command.opts` list.
///
/// This function provides the entry point for using the argument parsing logic.
///
/// # Returns
///
/// A `Command` struct containing the fully parsed command line: the program
/// name, the list of options and initial simple arguments (`opts`), and the
/// list of simple arguments found after the `"--"` separator (`args`).
pub fn get() -> Command {
    // Get an iterator over the command-line arguments provided by the environment.
    // This is generally efficient as it avoids collecting all arguments upfront
    // unless necessary (like with `.extend()`).
    let mut args_iter = env::args();

    // The first argument from the environment is conventionally the command name.
    // Use unwrap_or_default() to handle the unlikely case of no arguments being provided.
    let cmd_name = args_iter.next().unwrap_or_default();
    // Initialize the Command struct with the command name and empty lists.
    let mut command = Command::new(cmd_name);

    // Iterate through the rest of the arguments.
    // `while let Some(arg) = args_iter.next()` is a clean way to consume the iterator.
    while let Some(arg) = args_iter.next() {
        // Check if the current argument is the special "--" separator.
        if arg == "--" {
            // If "--" is found, collect all remaining arguments from the iterator
            // and add them directly to the `args` field.
            // `extend` is an efficient way to append all elements from another iterator.
            command.args.extend(args_iter);
            // Since all remaining arguments have been processed and added to `args`,
            // we can break out of the main parsing loop.
            break;
        }

        // If the argument is not "--", determine its type based on its format.
        let opt_type = determine_opt_type(&arg);

        // Use a match statement to handle parsing logic based on the determined type.
        match opt_type {
            OptionType::LongOpt => {
                // This is a long option (starts with "--").
                // Check if it contains an equals sign to indicate an associated value.
                if let Some((key, value)) = arg.split_once('=') {
                    // The long option has a value attached (e.g., "--data=value").
                    // Extract and parse the value part (which might be comma-separated).
                    let opt_values = parse_values(value);
                    // Add a new Option to the opts list.
                    // The opt_str is just the key part (e.g., "--data").
                    // The parsed values go into opt_values.
                    command.add_opt(Option {
                        opt_type: OptionType::LongOpt,
                        opt_str: key.to_string(), // Store only the flag key string
                        opt_values,
                    });
                } else {
                    // The long option does not have an attached value (e.g., "--help").
                    // Add a new Option to the opts list.
                    // The opt_str is the full argument string (e.g., "--help").
                    // opt_values is empty.
                    command.add_opt(Option {
                        opt_type: OptionType::LongOpt,
                        opt_str: arg,
                        opt_values: Vec::new(),
                    });
                }
            }
            OptionType::ShortOpt => {
                // This is a short option (starts with "-" and has length > 1).
                // Check if it might be a bundled set of short options (e.g., "-iv").
                if arg.len() > 2 {
                    // It's potentially bundled (e.g., "-iv" has length 3).
                    // Iterate over the characters *after* the initial hyphen.
                    for c in arg.chars().skip(1) {
                        // Skip the leading '-'
                        // For each character, create a separate ShortOpt.
                        // The opt_str is the character prefixed by a hyphen (e.g., "-i", "-v").
                        // Bundled short options typically don't take values in this format,
                        // so opt_values remains empty.
                        command.add_opt(Option {
                            opt_type: OptionType::ShortOpt,
                            opt_str: format!("-{}", c),
                            opt_values: Vec::new(),
                        });
                    }
                } else {
                    // It's a single short option (e.g., "-v").
                    // Add it as a single Option. The opt_str is the full argument.
                    // opt_values is empty.
                    command.add_opt(Option {
                        opt_type: OptionType::ShortOpt,
                        opt_str: arg,
                        opt_values: Vec::new(),
                    });
                }
            }
            OptionType::Simple => {
                // This is a simple argument (doesn't start with hyphen, or is just "-")
                // and it occurred *before* the "--" separator.
                // Add it to the opts list as per the defined behavior of this parser.
                // The opt_str is the full argument string. opt_values is empty.
                command.add_opt(Option {
                    opt_type: OptionType::Simple,
                    opt_str: arg,
                    opt_values: Vec::new(),
                });
            }
        }
    }

    // Return the fully populated Command struct.
    command
}

/// Retrieves the complete command line as a single string.
/// This includes the command name (program path) and all arguments,
/// joined by spaces. This can be useful for logging or debugging
/// the exact input received by the program.
///
/// It collects all arguments from the environment into a vector and then joins them.
///
/// # Returns
///
/// A `String` containing the command name and all arguments, separated by spaces.
pub fn cmd_str() -> String {
    env::args() // Get an iterator over the arguments
        .collect::<Vec<String>>() // Collect all arguments into a vector
        .join(" ") // Join the elements of the vector into a single string with spaces
}

/*
// Example usage in your main application file (e.g., main.rs):

// Make sure to add `colored = "2"` to your Cargo.toml dependencies
// and declare your module if it's in a separate file (e.g., `mod args;`
// if this code is in `src/args.rs`). Replace `your_crate_name`
// with the actual name of your crate defined in Cargo.toml.

// use your_crate_name::init;
// use your_crate_name::cmd_str;

fn main() {
    // Parse the command line arguments
    let command = your_crate_name::init();

    // Print the structured command information using the Display implementation
    println!("{}", command);

    // Optionally print the raw command string
    // println!("Raw command string: {}", cmd_str());
}

// Example Execution from your project directory:
// cargo run -- -iv file.txt --data=apple,banana --verbose -- positional1 --pos-flag another-arg

// Example Output (Actual program name will vary based on your build):
// Command: target/debug/your_program_name
// Options:
//   1. -i (Type: Short Option): Values: None
//   2. -v (Type: Short Option): Values: None
//   3. file.txt (Type: Simple): Values: None
//   4. --data (Type: Long Option): Values: [apple, banana]
//   5. --verbose (Type: Long Option): Values: None
// Arguments (-- after):
//   1. positional1
//   2. --pos-flag
//   3. another-arg

// Note how arguments after "--" are treated purely as arguments,
// even if they look like options (e.g., "--pos-flag").
// Simple arguments before "--" (like "file.txt") are included in the "Options" list.
*/
