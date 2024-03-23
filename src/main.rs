use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use regex::Regex;
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

    let header_files: HashSet<String> = files_with_ext(root, &[".h"], true);
    let source_files: HashSet<String> = files_with_ext(root, &[".h", ".cpp"], false);

    for source_path in source_files {
        // Read the file
        let file = File::open(source_path)?;
        let lines = BufReader::new(file).lines();

        for line in lines {
            let l = line?;

            // See if it matches an include
            let re = Regex::new(r#"#include "(?<file>.*?)""#)?;
            if let Some(matches) = re.captures(l.as_str()) {
                let include = matches.name("file").expect("not possible").as_str();

                if (header_files.contains(include)) {
                    // Rewrite it as relative
                    println!("Needs rewrite: {}", include);
                }
            }
        }
    }

    Ok(())
}

fn files_with_ext(project_path: &Path, exts: &[&str], remove_prefix: bool) -> HashSet<String> {
    let prefix_len = project_path.to_string_lossy().len() + 1; // +1 to compensate for leading \
    WalkDir::new(project_path).into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| {
            let name = f.file_name().to_string_lossy();
            exts.into_iter().any(|&ext| name.ends_with(ext))
        })
        .map(|f| f.into_path().to_string_lossy().to_string())
        .map(|f| {
            if remove_prefix {
                f.get(prefix_len..).expect("not possible").to_string()
            } else {
                f
            }
        })
        .collect()
}
