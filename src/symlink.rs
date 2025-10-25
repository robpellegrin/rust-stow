//use args::Args;
use std::{
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
                eprintln!("Error: target already exists at {}", link.display());
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
