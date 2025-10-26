use std::{
    env,
    fs::OpenOptions,
    io::{ErrorKind, Write},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use crate::args::Args;

pub fn create_symlink(target: &Path, link: &Path, args: &Args) {
    let link = if args.dotfiles {
        replace_dot_prefix(link)
    } else {
        link.to_path_buf()
    };

    // Do not symlink .rstow file.
    if path_contains_value(&link, ".rstow") {
        return;
    }

    match symlink(target, &link) {
        Ok(()) => {
            if args.verbose {
                println!(
                    "Symlink created: {} -> {}",
                    link.display(),
                    target.display()
                );
            }

            if let Err(e) = record_symlink(&link) {
                eprintln!("Warning: Failed to record symlink: {}", e);
            }
        }

        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                eprintln!("Error: target already exists at {}", link.display());
            } else {
                eprintln!("Error creating symlink: {}", e);
            }
        }
    }
}

/// Append the link path to a `.rstow` file in the current directory.
fn record_symlink(link: &Path) -> std::io::Result<()> {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Could not determine the home directory!");
            return Ok(());
        }
    };

    let log_path = home_dir.join(".rstow");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    writeln!(file, "{}", link.display())?;

    Ok(())
}

fn replace_dot_prefix(path: &Path) -> PathBuf {
    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        if let Some(rest) = filename.strip_prefix("dot-") {
            return path.with_file_name(format!(".{}", rest));
        }
    }

    path.to_path_buf()
}

fn path_contains_value(path: &Path, value: &str) -> bool {
    path.iter()
        .any(|component| component.to_string_lossy().contains(value))
}
