use std::fs;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
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
    /// Base path where ToB start scanning files
    #[arg(short = 'p', long)]
    path: Option<PathBuf>,
    /// A Glob style pattern to match files to scan
    #[arg(short = 'g', long)]
    path_pattern: Option<String>,
    /// Exclude todos from the diagnostic
    #[arg(short = 't', long, default_value = "true")]
    exclude_todos: bool,
    /// Exclude fixmes from the diagnostic
    #[arg(short = 'f', long, default_value = "false")]
    exclude_fixmes: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(std::env::current_dir()?);
    let glob_pattern = cli.path_pattern.unwrap_or("**".to_string());
    let glob = globset::Glob::new(&glob_pattern)?.compile_matcher();
    let mut diagnostics = Diagnostics::default();
    for entry in ignore::Walk::new(path) {
        let entry = entry?;
        if !glob.is_match(entry.path()) {
            continue
        }

        let Some(filename) = entry.path().file_name() else {
            continue;
        };

        let filename = filename.to_string_lossy();
        let filename = filename.as_ref();
        let file = fs::read_to_string(entry.path())?;
        parsers::parse(&file, filename, &mut diagnostics)?;
    }

    println!("{}", diagnostics);

    if diagnostics.is_err() {
        exit(1)
    } else {
        Ok(())
    }
}
