use std::fs::{File, remove_file};
use std::io::{self, BufRead};
use std::path::Path;

/// Reads `.rstow`, deletes each path listed inside, and then removes `.rstow` itself.
pub fn cleanup_symlinks(home_dir: &Path) -> io::Result<()> {
    let rstow_path = home_dir.join(".rstow");

    if !rstow_path.exists() {
        eprintln!("No .rstow file found — nothing to clean up.");
        return Ok(());
    }

    let file = File::open(&rstow_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let path_str = line?;
        let path = Path::new(&path_str);

        if path.exists() {
            match remove_file(path) {
                Ok(()) => println!("Removed symlink: {}", path.display()),
                Err(e) => eprintln!("Failed to remove {}: {}", path.display(), e),
            }
        } else {
            eprintln!("Path not found: {}", path.display());
        }
    }

    match remove_file(rstow_path) {
        Ok(()) => println!("Removed .rstow file."),
        Err(e) => eprintln!("Failed to remove .rstow file: {}", e),
    }

    Ok(())
}
