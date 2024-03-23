use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

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
    let root: &Path = args.project_path.as_path();

    println!("Project path: {:?}", args.project_path);

    let header_files: HashSet<PathBuf> = files_with_ext(root, &[".h"]);
    let source_files: HashSet<PathBuf> = files_with_ext(root, &[".h", ".cpp"]);

    for source_file in source_files {
        // Read the file

        // Parse each #include

        // See if it matches a header file

        // Rewrite it as relative
    }

    Ok(())
}

fn files_with_ext(project_path: &Path, exts: &[&str]) -> HashSet<PathBuf> {
    WalkDir::new(project_path).into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| {
            let name = f.file_name().to_string_lossy();
            exts.into_iter().any(|&ext| name.ends_with(ext))
        })
        .map(|f| f.into_path())
        .collect()
}
