use std::{io, path::Path};

/// Reads `.rstow`, deletes each path listed inside, and then removes `.rstow` itself.
pub fn cleanup_symlinks(home_dir: &Path) -> io::Result<()> {
    println!("Home dir: {}", home_dir.display());
    println!("Feature not yet implemented");

    Ok(())
}
