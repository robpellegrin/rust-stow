use args::Args;
use clap::Parser;
use rayon::prelude::*;
use std::{
    env, fs,
    io::{self, ErrorKind},
    os::unix::fs::symlink,
    path::Path,
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
        if args.verbose {
            println!("{}", item);
        }
    });

    let target = Path::new("target_file.txt");
    let symlink_name = Path::new("my_symlink.txt");

    let symlink_path = home_dir.join(symlink_name);
    create_symlink(target, &symlink_path)?;

    Ok(())
}

fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    match symlink(target, link) {
        Ok(()) => println!(
            "Symlink created: {} -> {}",
            link.display(),
            target.display()
        ),
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                println!("Error: A file already exists at {}", link.display());
            } else {
                println!("Error creating symlink: {}", e);
            }
        }
    }

    Ok(())
}

fn list_current_dir() -> io::Result<Vec<String>> {
    let current_dir = env::current_dir()?;

    let entries = fs::read_dir(current_dir)?
        .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
        .collect();

    Ok(entries)
}
