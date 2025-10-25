use std::os::unix::fs::symlink;
use std::{env, io::ErrorKind, path::Path};

mod args;

fn main() -> std::io::Result<()> {
    // Get the home directory
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            println!("Could not determine the home directory!");
            return Ok(());
        }
    };

    // Example of creating symlinks in the home directory
    let target = Path::new("target_file.txt"); // Adjust your target file path
    let symlink_name = Path::new("my_symlink.txt"); // Symlink name in the home directory

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
