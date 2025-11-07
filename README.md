# rust-stow
rust-stow is a fast and lightweight tool for managing dotfiles and software packages in your home directory through symbolic linking. Offering reliable directory symlink management.

## Roadmap / TODO
 - ~~symlink into existing directories~~
 - ~~Ignore git files/dirs~~
 - ~~.rstow-ignore~~
 - `--force` flag to overwrite confilcting files.

## Usage
```
rstow --help
A simple, fast tool for dotfile managment.

Usage: rstow [OPTIONS]

Options:
  -n, --simulate  Do no perform any operations that modify the filesystem; merely show what would happen
  -v, --verbose   Send verbose output to standard error describing what rstow is doing
  -d, --unstow    Removed all stowed items
      --adopt     (Use with care!)  Import existing files into stow package
      --dotfiles  Stow packages that start with "dot-" and not "."
  -h, --help      Print help
  -V, --version   Print version
```
