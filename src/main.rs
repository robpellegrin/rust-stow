use args::Args;
use clap::Parser;
use rayon::prelude::*;
use std::{
    env, fs,
    io::{self, ErrorKind},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

mod args;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            println!("Could not determine the home directory!");
            return Ok(());
        }
    };

    list_current_dir()?.par_iter().for_each(|item| {
        if let Some(filename) = item.file_name() {
            let new_path = home_dir.join(filename);
            create_symlink(&item, &new_path, &args);
        }
    });

    Ok(())
}

fn create_symlink(target: &Path, link: &Path, args: &Args) {
    match symlink(target, link) {
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
                println!("Error: target already exists at {}", link.display());
            } else {
                println!("Error creating symlink: {}", e);
            }
        }
    }
}

fn list_current_dir() -> io::Result<Vec<PathBuf>> {
    let current_dir = env::current_dir()?;

    let entries = fs::read_dir(&current_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    Ok(entries)
}
