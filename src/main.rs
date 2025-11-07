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
use dirs_next::home_dir;
use rayon::prelude::*;
use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

mod args;
mod symlink;
mod unstow;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let ignore_files = read_ignore_file()?;

    let home_dir = match home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Could not determine the user's home directory.");
            return Ok(());
        }
    };

    if args.unstow {
        unstow::cleanup_symlinks(&home_dir)?;
        return Ok(());
    }

    list_current_dir()?.par_iter().for_each(|item| {
        if let Some(filename) = item.file_name() {
            if !is_path_ignored(&ignore_files, &filename.to_string_lossy()) {
                let new_path = home_dir.join(filename);
                symlink::create_symlink(&item, &new_path, &args);
            }
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

/// Reads the '.rstow-ignore' for files and directories that should be skipped.
/// Returns an empty Vec if the file does not exist or cannot be read.
fn read_ignore_file() -> io::Result<Vec<String>> {
    let ignore_path = Path::new(".rstow-ignore");

    if !ignore_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(&ignore_path)?;
    let reader = BufReader::new(file);

    let mut paths = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            paths.push(trimmed.to_string());
        }
    }

    Ok(paths)
}

fn is_path_ignored(paths: &Vec<String>, path_to_check: &str) -> bool {
    paths.contains(&path_to_check.to_string())
}
