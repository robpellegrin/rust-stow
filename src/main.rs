use args::Args;
use clap::Parser;
use rayon::prelude::*;
use std::{env, fs, io, path::PathBuf};

mod args;
mod symlink;
mod unstow;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.unstow {
        unstow::cleanup_symlinks()?;
    }

    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Could not determine the home directory!");
            return Ok(());
        }
    };

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
