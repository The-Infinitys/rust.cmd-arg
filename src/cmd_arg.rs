use colored::Colorize;
use std::env;
use std::fmt;

/// Represents the classification of a command-line argument based on its format.
/// This enum distinguishes between simple arguments, short options, and long options.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
    /// A standard positional argument or any argument that does not conform to option syntax.
    /// Includes strings not starting with a hyphen (e.g., `input.txt`), a single hyphen (`-`),
    /// or arguments after the `--` separator.
    Simple,

    /// An argument starting with a single hyphen followed by one or more characters (e.g., `-v`).
    /// Supports bundled short options (e.g., `-abc` is parsed as `-a`, `-b`, `-c`).
    ShortOpt,

    /// An argument starting with two hyphens followed by one or more characters (e.g., `--verbose`).
    /// May include values attached via an equals sign (e.g., `--output=results.txt`).
    LongOpt,
}

/// Implements the `Default` trait for `OptionType`.
impl Default for OptionType {
    /// Returns the default variant of `OptionType`, which is `Simple`.
    ///
    /// # Returns
    ///
    /// `OptionType::Simple`, as it represents the most basic form of a command-line argument.
    fn default() -> Self {
        OptionType::Simple
    }
}

/// Implements the `Display` trait for `OptionType` to provide a human-readable representation.
impl fmt::Display for OptionType {
    /// Formats the `OptionType` variant as a colored string for console output.
    ///
    /// - `Simple` is displayed in purple.
    /// - `ShortOpt` is displayed in yellow.
    /// - `LongOpt` is displayed in cyan.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the output to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the formatting was successful.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            OptionType::Simple => "Simple".purple(),
            OptionType::ShortOpt => "Short Option".yellow(),
            OptionType::LongOpt => "Long Option".cyan(),
        };
        write!(f, "{}", type_str)
    }
}

/// Holds the parsed information for a single command-line argument classified as an option
/// or a simple argument appearing before the `--` separator.
#[derive(Debug, Clone)]
pub struct Option {
    /// The classification of this argument's format (`Simple`, `ShortOpt`, or `LongOpt`).
    pub opt_type: OptionType,

    /// The string representation of the option or simple argument.
    /// - For `ShortOpt` (e.g., `-v`), this is the full string (e.g., `"-v"`).
    /// - For `LongOpt` without a value (e.g., `--help`), this is the full string (e.g., `"--help"`).
    /// - For `LongOpt` with a value (e.g., `--data=val`), this is the flag part (e.g., `"--data"`).
    /// - For `Simple` arguments (e.g., `file.txt`), this is the full string.
    pub opt_str: String,

    /// A list of values associated with the option, if any.
    /// - For `LongOpt` with values (e.g., `--data=v1,v2`), contains the parsed values (e.g., `["v1", "v2"]`).
    /// - Empty for `Simple`, `ShortOpt`, or `LongOpt` without values.
    pub opt_values: Vec<String>,
}

/// Implements the `Default` trait for `Option`.
impl Default for Option {
    /// Returns a default `Option` instance with empty or default values.
    ///
    /// # Returns
    ///
    /// An `Option` with:
    /// - `opt_type`: `OptionType::Simple` (via `OptionType::default()`).
    /// - `opt_str`: An empty string.
    /// - `opt_values`: An empty vector.
    fn default() -> Self {
        Option {
            opt_type: OptionType::default(),
            opt_str: String::new(),
            opt_values: Vec::new(),
        }
    }
}

/// Implements the `Display` trait for `Option` to provide a human-readable representation.
impl fmt::Display for Option {
    /// Formats the `Option` struct as a colored string for console output.
    ///
    /// - The `opt_str` is displayed in magenta.
    /// - The `opt_type` is formatted using its own `Display` implementation.
    /// - The `opt_values` are shown as a comma-separated list in green (or "None" in red if empty).
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the output to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the formatting was successful.
    ///
    /// # Example
    ///
    /// An `Option` with `opt_str = "--data"`, `opt_type = LongOpt`, and `opt_values = ["v1", "v2"]`
    /// might be formatted as:
    /// ```
    /// --data (Type: Long Option): Values: [v1, v2]
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values = if self.opt_values.is_empty() {
            "None".red().to_string()
        } else {
            format!("[{}]", self.opt_values.join(", ").green())
        };
        write!(
            f,
            "{} ({}: {}): {}: {}",
            self.opt_str.magenta(),
            "Type".cyan(),
            self.opt_type,
            "Values".cyan(),
            values
        )
    }
}

/// Represents the complete structured result of parsing the command line.
/// It separates the program name, options/initial simple arguments, and arguments after `--`.
#[derive(Debug)]
pub struct Command {
    /// The name of the executable program, typically the first argument from the environment.
    pub cmd_name: String,

    /// A vector of parsed options (`ShortOpt`, `LongOpt`) and simple arguments before `--`.
    pub opts: Vec<Option>,

    /// A vector of simple arguments appearing after the `--` separator.
    pub args: Vec<String>,
}

/// Implements the `Default` trait for `Command`.
impl Default for Command {
    /// Returns a default `Command` instance with empty fields.
    ///
    /// # Returns
    ///
    /// A `Command` with:
    /// - `cmd_name`: An empty string.
    /// - `opts`: An empty vector.
    /// - `args`: An empty vector.
    fn default() -> Self {
        Command {
            cmd_name: String::new(),
            opts: Vec::new(),
            args: Vec::new(),
        }
    }
}

/// Implements the `Display` trait for `Command` to provide a human-readable representation.
impl fmt::Display for Command {
    /// Formats the `Command` struct as a colored, structured string for console output.
    ///
    /// - Displays the command name in blue with a bold cyan "Command" label.
    /// - Lists all options (from `opts`) with their type and values, or indicates none were found.
    /// - Lists all arguments after `--` (from `args`), or indicates none were found.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the output to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the formatting was successful.
    ///
    /// # Example
    ///
    /// For a command like `program -v file.txt --data=apple,banana -- positional1`, the output might be:
    /// ```
    /// Command: program
    /// Options:
    ///   1. -v (Type: Short Option): Values: None
    ///   2. file.txt (Type: Simple): Values: None
    ///   3. --data (Type: Long Option): Values: [apple, banana]
    /// Arguments (-- after):
    ///   1. positional1
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {}", "Command".cyan().bold(), self.cmd_name.blue())?;
        writeln!(f, "{}:", "Options".green().bold())?;
        if self.opts.is_empty() {
            writeln!(f, "  {}", "No Options provided (before --).".red())?;
        } else {
            for (i, opt) in self.opts.iter().enumerate() {
                writeln!(f, "  {}. {}", (i + 1).to_string().bold(), opt)?;
            }
        }
        writeln!(f, "{}:", "Arguments (-- after)".green().bold())?;
        if self.args.is_empty() {
            writeln!(f, "  {}", "No arguments provided after --.".red())?;
        } else {
            for (i, arg) in self.args.iter().enumerate() {
                writeln!(f, "  {}. {}", (i + 1).to_string().bold(), arg.blue())?;
            }
        }
        Ok(())
    }
}

impl Command {
    /// Creates a new `Command` instance with the specified command name and empty vectors.
    ///
    /// # Arguments
    ///
    /// * `cmd_name` - The name of the command (e.g., the program name).
    ///
    /// # Returns
    ///
    /// A `Command` instance with the given `cmd_name`, and empty `opts` and `args` vectors.
    fn new(cmd_name: String) -> Self {
        Command {
            cmd_name,
            opts: Vec::new(),
            args: Vec::new(),
        }
    }

    /// Adds a parsed `Option` to the internal `opts` vector.
    ///
    /// # Arguments
    ///
    /// * `opt` - The `Option` struct to add, representing a parsed argument before `--`.
    fn add_opt(&mut self, opt: Option) {
        self.opts.push(opt);
    }
}

/// Determines the classification of a command-line argument based on its format.
///
/// This function is used for arguments before the `--` separator. Arguments after `--`
/// are always treated as `Simple` and are not passed to this function.
///
/// # Arguments
///
/// * `arg` - The command-line argument to classify.
///
/// # Returns
///
/// An `OptionType` indicating whether the argument is `Simple`, `ShortOpt`, or `LongOpt`.
///
/// # Examples
///
/// ```
/// assert_eq!(determine_opt_type("file.txt"), OptionType::Simple);
/// assert_eq!(determine_opt_type("-v"), OptionType::ShortOpt);
/// assert_eq!(determine_opt_type("--verbose"), OptionType::LongOpt);
/// assert_eq!(determine_opt_type("-"), OptionType::Simple);
/// ```
fn determine_opt_type(arg: &str) -> OptionType {
    if arg.starts_with("--") {
        OptionType::LongOpt
    } else if arg.starts_with("-") && arg.len() > 1 {
        OptionType::ShortOpt
    } else {
        OptionType::Simple
    }
}

/// Parses a comma-separated string of values into a vector of trimmed strings.
///
/// Used for processing values in long options with `=` (e.g., `--data=v1,v2`).
/// Empty strings (e.g., from `,,` or trailing commas) are filtered out.
///
/// # Arguments
///
/// * `value` - The string containing comma-separated values.
///
/// # Returns
///
/// A `Vec<String>` of trimmed, non-empty values. Returns an empty vector if the input is empty or contains only whitespace/commas.
///
/// # Examples
///
/// ```
/// assert_eq!(parse_values("v1,v2"), vec!["v1", "v2"]);
/// assert_eq!(parse_values("v1, v2, "), vec!["v1", "v2"]);
/// assert_eq!(parse_values(""), vec![] as Vec<String>);
/// ```
fn parse_values(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parses the command-line arguments from the environment into a structured `Command`.
///
/// The first argument is the command name. Arguments before `--` are parsed as options
/// or simple arguments and stored in `opts`. Arguments after `--` are stored in `args`.
///
/// # Returns
///
/// A `Command` struct containing:
/// - The command name.
/// - A vector of parsed options/simple arguments before `--`.
/// - A vector of arguments after `--`.
///
/// # Examples
///
/// For a command like `program -iv file.txt --data=apple,banana --verbose -- positional1 --pos-flag`:
/// - `cmd_name` will be `"program"`.
/// - `opts` will include `-i`, `-v`, `file.txt`, `--data` (with values `["apple", "banana"]`), and `--verbose`.
/// - `args` will include `["positional1", "--pos-flag"]`.
pub fn get() -> Command {
    let mut args_iter = env::args();
    let cmd_name = args_iter.next().unwrap_or_default();
    let mut command = Command::new(cmd_name);

    while let Some(arg) = args_iter.next() {
        if arg == "--" {
            command.args.extend(args_iter);
            break;
        }

        let opt_type = determine_opt_type(&arg);

        match opt_type {
            OptionType::LongOpt => {
                if let Some((key, value)) = arg.split_once('=') {
                    let opt_values = parse_values(value);
                    command.add_opt(Option {
                        opt_type: OptionType::LongOpt,
                        opt_str: key.to_string(),
                        opt_values,
                    });
                } else {
                    command.add_opt(Option {
                        opt_type: OptionType::LongOpt,
                        opt_str: arg,
                        opt_values: Vec::new(),
                    });
                }
            }
            OptionType::ShortOpt => {
                if arg.len() > 2 {
                    for c in arg.chars().skip(1) {
                        command.add_opt(Option {
                            opt_type: OptionType::ShortOpt,
                            opt_str: format!("-{}", c),
                            opt_values: Vec::new(),
                        });
                    }
                } else {
                    command.add_opt(Option {
                        opt_type: OptionType::ShortOpt,
                        opt_str: arg,
                        opt_values: Vec::new(),
                    });
                }
            }
            OptionType::Simple => {
                command.add_opt(Option {
                    opt_type: OptionType::Simple,
                    opt_str: arg,
                    opt_values: Vec::new(),
                });
            }
        }
    }

    command
}

/// Retrieves the complete command line as a single string, including the command name and all arguments.
///
/// # Returns
///
/// A `String` containing all command-line arguments joined by spaces.
///
/// # Examples
///
/// For a command like `program -v file.txt`, this returns:
/// ```
/// "program -v file.txt"
/// ```
pub fn cmd_str() -> String {
    env::args().collect::<Vec<String>>().join(" ")
}
