use clap::{Parser, Subcommand};
use clap::{ArgGroup, Args};
use std::ffi::OsString;
use std::path::PathBuf;

/// Search through text or files using patterns.
///
/// Exits with status 0 if the search found at least one match, otherwise 1.
#[derive(Debug, Parser)]
#[command(
    version,
    long_about,
    groups([
        ArgGroup::new("input_group")
            .args(["stdin", "path", "trailing"])
            .multiple(false),
        ArgGroup::new("output_group")
            .args([
                "lines", "line_numbers", "tabular",
                "matches", "split", "replace"
            ])
            .multiple(false)
            .conflicts_with("test")
    ])
)]
pub struct Cli {
    /// Ignore difference in case when matching.
    #[arg(short = 'c', long)]
    pub ignore_case: bool,

    /// Start the search from the given index.
    #[arg(short = 'i', long)]
    pub start_index: Option<usize>,

    /// Test if a given string is a valid regex without performing a search.
    ///
    /// If passed, ignores input and output.
    #[arg(
        short = 'Q',
        long,
        value_name = "REGEX",
        conflicts_with_all = [
            "ignore_case", "start_index", "quiet",
            "find", "global", "global_all", "test",
            "stdin", "path",
            "lines", "line_numbers", "tabular", "matches",
            "split", "replace", "count",
            "pattern_flag", "pattern_arg", "literal_string"
        ]
    )]
    pub test_regex: Option<OsString>,

    /// Disable all output. The exit status can be used to find the result of
    /// the search.
    #[arg(short, long, conflicts_with_all = ["global", "global_all"])]
    pub quiet: bool,

    #[command(flatten, next_help_heading = "Search mode")]
    pub mode: Mode,

    #[command(flatten, next_help_heading = "Input")]
    pub input: Input,

    #[command(flatten, next_help_heading = "Output")]
    pub output: Output,

    #[command(flatten, next_help_heading = "Pattern")]
    pub pattern: Pattern,

    /// Use trailing arguments as the input string and match against that.
    #[arg(
        trailing_var_arg = true,
        value_name = "TRAILING_INPUT",
        help_heading = "Input"
    )]
    pub trailing: Vec<OsString>,
}

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct Mode {
    /// Return the first match. (default)
    #[arg(long)]
    pub find: bool,

    /// Return all non-overlapping matches.
    #[arg(short, long)]
    pub global: bool,

    /// Return all possible matches, including overlapping ones.
    #[arg(short = 'G', long)]
    pub global_all: bool,

    /// Test whether the whole input string matches. If it does, print the
    /// whole string.
    #[arg(short = 'T', long)]
    pub test: bool,
}

#[derive(Debug, Args)]
pub struct Input {
    /// Match against stdin. (default)
    #[arg(long)]
    pub stdin: bool,

    /// Match against content of the specified file or directory.
    #[arg(short = 'f', long)]
    pub path: Option<PathBuf>,

    /// Recursively search through subdirectories.
    #[arg(short, long, requires = "path")]
    pub recursive: bool,

    /// Follow symlinks to search recursively.
    #[arg(long, requires = "path")]
    pub follow_symlinks: bool,
}

#[derive(Debug, Args)]
#[group()]
pub struct Output {
    /// Print each line with at least one match. (default)
    #[arg(long)]
    pub lines: bool,

    /// Print each line with at least one match, with line numbers.
    #[arg(short = 'n', long)]
    pub line_numbers: bool,

    /// Print all matches in tabular form.
    ///
    /// The columns of the table are as follows: (file path); match index;
    /// match_length; line number; column number; match content.
    ///
    /// The file path column is only included if '--path' and '--recursive' are
    /// passed. Each field is separated by the string passed to '--separator',
    /// or '\t' by default.
    #[arg(short = 'X', long)]
    pub tabular: bool,

    /// The separator used between fields when printing the output of '--tabular'.
    ///
    /// If SEPARATOR is a single character, and unless '--no-escape' is passed,
    /// instances of that character within the table are escaped with a
    /// backslash.
    #[arg(long, requires = "tabular")]
    pub separator: Option<OsString>,

    /// Disable escaping non-separator instances of the separator character in
    /// '--tabular'
    #[arg(long, requires = "tabular")]
    pub no_escape: bool,

    /// Print each matched string.
    #[arg(short = 'M', long)]
    pub matches: bool,

    /// Split the string by each match, and print each split part.
    #[arg(short = 'S', long)]
    pub split: bool,

    /// Replace each match by a given string, and print the result.
    #[arg(
        short = 'R',
        long,
        conflicts_with = "global_all",
        value_name = "REPLACE_STRING"
    )]
    pub replace: Option<OsString>,

    /// Print the number of matches.
    #[arg(short = 'C', long)]
    pub count: bool,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct Pattern {
    /// A regex pattern.
    #[arg(value_name = "PATTERN")]
    pub pattern_arg: Option<OsString>,

    /// A regex pattern.
    #[arg(
        short,
        long = "pattern",
        value_name = "PATTERN",
        allow_hyphen_values = true
    )]
    pub pattern_flag: Option<OsString>,

    /// A literal string.
    #[arg(short = 'P', long, value_name = "PATTERN")]
    pub literal_string: Option<OsString>,
}
