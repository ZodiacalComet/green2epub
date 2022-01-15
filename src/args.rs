#![deny(missing_docs)]
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author)]
/// Create an EPUB from text files in greentext format.
pub struct Args {
    /// Title of the green.
    #[clap(short, long)]
    pub title: String,
    /// Name of the author.
    #[clap(short, long)]
    pub author: String,
    /// Cover image to use.
    #[clap(short, long)]
    pub cover: Option<String>,
    /// Green subjects/tags.
    #[clap(short, long)]
    pub subjects: Option<Vec<String>>,
    /// Color of the green highlight.
    #[clap(long, default_value = "#2CAF26")]
    pub green_color: String,
    /// Color of the spoiler highlight.
    #[clap(long, default_value = "#000")]
    pub spoiler_color: String,
    /// Shows verbose output, can be used multiple times to set level of verbosity.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
    /// Path for the generated epub file.
    #[clap(short, long)]
    pub output: String,
    /// Text files in "green" format to convert.
    pub files: Vec<String>,
}
