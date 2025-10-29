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
use rayon::prelude::*;
use std::{
    env,
    fs::{self, remove_file},
    io,
    path::{Path, PathBuf},
};

pub fn cleanup_symlinks(home_dir: &Path) -> io::Result<()> {
    list_current_dir()?.par_iter().for_each(|item| {
        if let Some(filename) = item.file_name() {
            let _ = remove_file(home_dir.join(filename));
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
