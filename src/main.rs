use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    /// The root of the project to convert
    project_path: std::path::PathBuf
}

fn main() -> Result<()> {
    let args: Cli = Cli::try_parse()?;

    println!("Project path: {:?}", args.project_path);

    let headerFiles: HashSet<PathBuf> = WalkDir::new(args.project_path).into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_name().to_string_lossy().ends_with(".h"))
        .map(|f| f.into_path())
        .collect();

    for f in headerFiles.into_iter() {
        println!("{:?}", f);
    }

    Ok(())
}
