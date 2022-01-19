#![deny(missing_docs)]
use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Debug, Clone, Copy)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl Default for Color {
    fn default() -> Color {
        Color::Auto
    }
}

#[derive(Parser, Debug)]
#[clap(version, author)]
/// Create an EPUB from text files in greentext format.
pub struct Args {
    /// Title of the greentext.
    #[clap(short, long, forbid_empty_values(true))]
    pub title: String,
    /// Name of the author.
    #[clap(short, long, forbid_empty_values(true))]
    pub author: String,
    /// Cover image to use.
    #[clap(short, long, value_name = "FILE", forbid_empty_values(true))]
    pub cover: Option<String>,
    /// Greentext subjects/tags.
    ///
    /// Can be used multiple times to set more than one.
    #[clap(short, long = "subject", alias = "tag", value_name = "SUBJECT", forbid_empty_values(true))]
    pub subjects: Option<Vec<String>>,
    /// Color of the green highlight.
    #[clap(long, default_value = "#2CAF26", value_name = "COLOR", forbid_empty_values(true))]
    pub green_color: String,
    /// Color of the spoiler highlight.
    #[clap(long, default_value = "#000", value_name = "COLOR", forbid_empty_values(true))]
    pub spoiler_color: String,
    /// Supress all output.
    #[clap(short, long, conflicts_with("verbose"))]
    pub quiet: bool,
    /// Shows verbose output, can be used multiple times to set level of verbosity.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
    /// When to use colors.
    #[clap(long, arg_enum, default_value_t, value_name = "WHEN")]
    pub color: Color,
    /// Path for the generated epub file.
    #[clap(short, long, value_name = "PATH", forbid_empty_values(true))]
    pub output: String,
    /// Text files in greentext format to convert.
    #[clap(value_name = "FILE", forbid_empty_values(true))]
    pub files: Vec<String>,
}
