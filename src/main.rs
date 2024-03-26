use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

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

    println!("Project path: {:?}", args.project_path);

    // All header and source files
    let source_files: Vec<PathBuf> = WalkDir::new(&args.project_path).into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| {
            match f.path().extension() {
                Some(ext) => ext == "cpp" || ext == "h" || ext == "inl",
                _ => false,
            }
        })
        .map(|f| f.path().to_path_buf())
        .collect();

    // The header files. Strip off the project directory so paths are relative
    let header_files: HashSet<PathBuf> = source_files.clone().into_iter()
        .filter(|f| {
            match f.extension() {
                Some(ext) => ext == "h" || ext == "inl",
                _ => false,
            }
        })
        .map(|f| f.strip_prefix(&args.project_path).expect("not possible").to_path_buf())
        .collect();

    for source_path in source_files {
        let mut modified_content = String::new();
        let mut did_modify = false;

        // Read the file
        let file = File::open(&source_path)?;
        let lines = BufReader::new(file).lines();
        for line_result in lines {
            let mut line = line_result?;

            // See if it matches an include
            let re = Regex::new(r#"#include "(?<file>.*?)"(?<remainder>.*)"#)?;
            if let Some(matches) = re.captures(line.as_str()) {
                let include_str = matches.name("file").expect("not possible").as_str();
                let remainder = matches.name("remainder").expect("not possible").as_str();
                let include = Path::new(include_str);

                if header_files.contains(include) {
                    // Get the source file's directory and strip the project root, so it matches #include format
                    let dir = dir(&source_path).strip_prefix(&args.project_path)?.to_path_buf();

                    // Rewrite it as relative
                    let rel_include = relative_path(&dir, include)?;
                    let rel_include_str = rel_include.to_string_lossy();

                    // Set the new line
                    if include != rel_include {
                        line = format!(r#"#include "{rel_include_str}"{remainder}"#);
                        did_modify = true;
                    }
                }
            }

            modified_content += format!("{line}\n").as_str();
        }

        if did_modify {
            println!("{modified_content}");
        }
    }

    Ok(())
}

fn relative_path(from_dir: &Path, to_file: &Path) -> Result<PathBuf> {
    // Find common prefix
    let mut prefix = PathBuf::new();
    for part in to_file.iter() {
        prefix.push(part);
        if !from_dir.starts_with(&prefix) {
            prefix.pop();
            break;
        }
    }

    let mut path = PathBuf::new();

    // Number of ..'s
    let ascend_count = from_dir.iter().count() - prefix.iter().count();
    for _ in 0..ascend_count {
        path.push("..");
    }

    // Sub-path from prefix
    let from_prefix = to_file.strip_prefix(&prefix).expect("not possible");
    path.push(from_prefix);

    Ok(path)
}

fn dir(file: &Path) -> PathBuf {
    let str = file.parent().unwrap_or(Path::new("/")).to_owned();
    PathBuf::from(str)
}
