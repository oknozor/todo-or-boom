use std::fs;
use std::path::PathBuf;

use clap::Parser;
use wax::Glob;

use crate::diagnostics::Diagnostics;

mod diagnostics;
mod parsers;

#[derive(Debug, Parser)]
#[command(
    version,
    name = "tob",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
pub struct Cli {
    #[arg(short = 'p', long)]
    path: Option<PathBuf>,
    #[arg(short = 'g', long)]
    path_pattern: Option<String>,
    #[arg(short = 'd', long)]
    comment_delimiters: Vec<String>,
    #[arg(short = 't', long, default_value = "true")]
    include_todos: bool,
    #[arg(short = 'f', long, default_value = "false")]
    include_fixme: bool,
    #[arg(short = 'c', long, default_value = "true")]
    case_sensitive: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(std::env::current_dir()?);
    let glob_pattern = cli.path_pattern.unwrap_or("**".to_string());
    let mut comment_delimiters = cli.comment_delimiters;

    // TODO: pest
    if comment_delimiters.is_empty() {
        comment_delimiters.push("//".to_string());
        comment_delimiters.push("/*".to_string());
        comment_delimiters.push("*/".to_string());
    }

    // Comment
    let glob = Glob::new(&glob_pattern).unwrap();
    let mut diagnostics = Diagnostics::default();
    for entry in glob.walk(path) {
        let entry = entry.unwrap();
        let Some(filename) = entry.path().file_name() else {
            continue;
        };

        let filename = filename.to_string_lossy();
        let filename = filename.as_ref();
        let file = fs::read_to_string(entry.path())?;
        parsers::parse(&file, filename, &mut diagnostics)?;
    }

    println!("{}", diagnostics);

    Ok(())
}
