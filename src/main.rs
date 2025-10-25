mod args;
use std::os::unix::fs::symlink;
use std::path::Path;

fn main() {
    create_link().expect("Failed to create symlink");
}

fn create_link() -> std::io::Result<()> {
    let target = Path::new("target_file.txt");
    let link = Path::new("my_symlink.txt");

    symlink(target, link)?;
    println!(
        "Symlink created: {} -> {}",
        link.display(),
        target.display()
    );

    Ok(())
}
