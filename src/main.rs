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
use args::Args;
use clap::Parser;
use rayon::prelude::*;
use std::{env, fs, io, path::PathBuf};

mod args;
mod symlink;
mod unstow;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Could not determine the home directory!");
            return Ok(());
        }
    };

    if args.unstow {
        unstow::cleanup_symlinks(&home_dir)?;
        return Ok(());
    }

    list_current_dir()?.par_iter().for_each(|item| {
        if let Some(filename) = item.file_name() {
            let new_path = home_dir.join(filename);
            symlink::create_symlink(&item, &new_path, &args);
        }
    });

    Ok(())
}

fn list_current_dir() -> io::Result<Vec<PathBuf>> {
    let current_dir = env::current_dir()?;

    let entries = fs::read_dir(&current_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    Ok(entries)
}
