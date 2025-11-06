///
/// =====================================================================
/// Project Name: rust-stow
/// Description: A fast and lightweight tool for managing dotfiles and
///     software packages in your home directory through symbolic linking.
/// Author: Robert Pellegrin
/// Date: 2025-10-10
/// Version: 0.0.1
/// License: MIT
/// Repository:
/// =====================================================================
///
use std::{
    fs::read_dir,
    io::ErrorKind,
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

    match symlink(target, &link) {
        Ok(()) => {
            if args.verbose {
                println!(
                    "Symlink created: {} -> {}",
                    link.display(),
                    target.display()
                );
            }
        }

        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                if link.is_dir() && target.is_dir() {
                    // Recursively descend into subdirectories
                    match read_dir(target) {
                        Ok(entries) => {
                            for entry in entries.flatten() {
                                let sub_target = entry.path();
                                let sub_link = link.join(entry.file_name());
                                create_symlink(&sub_target, &sub_link, args);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading directory {}: {}", target.display(), err);
                        }
                    }
                } else {
                    eprintln!("Error: target already exists at {}", link.display());
                }
            } else {
                eprintln!("Error creating symlink: {}", e);
            }
        }
    }
}

fn replace_dot_prefix(path: &Path) -> PathBuf {
    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        if let Some(rest) = filename.strip_prefix("dot-") {
            return path.with_file_name(format!(".{}", rest));
        }
    }

    path.to_path_buf()
}
