use std::fs;
use anyhow::{Result, Context};
use clap::Parser;
use sift::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let haystack = fs::read_to_string(&cli.path)
    .with_context(|| format!("failed to read {}", cli.path.display()))?;

    let matches = search_pattern(&cli.pattern, &haystack);
    for (_, line) in matches {
        println!("{line}");
    }
    
    Ok(())
}
