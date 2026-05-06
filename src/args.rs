use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "du",
    about = "Display disk usage of files and directories",
    version
)]
pub struct Args {
    /// Directory or file to analyze (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Maximum depth of subdirectories to display
    #[arg(short = 'd', long = "depth", value_name = "N")]
    pub depth: Option<usize>,

    /// Print sizes in human-readable format (e.g., 1.5 MB)
    #[arg(short = 'H', long = "human-readable")]
    pub human_readable: bool,

    /// Display only a total for the specified directory
    #[arg(short = 's', long = "summarize", conflicts_with = "depth")]
    pub summarize: bool,
}
