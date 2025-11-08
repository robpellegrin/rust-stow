# rust-stow
rust-stow is a fast and lightweight tool for managing dotfiles and software packages in your home directory through symbolic linking. Offering reliable directory symlink management.

## Roadmap / TODO
 - ~~symlink into existing directories~~
 - ~~Ignore git files/dirs~~
 - ~~.rstow-ignore~~
 - ~~`--force` flag to overwrite confilcting files.~~
 - --adopt flag to import existing files.

## Building & Installation
Using the makefile:
```
make build && make install
```
This compiles rstow and creates a symlink at `~/.cargo/bin`

## Usage
```
rstow --help
A simple, fast tool for dotfile managment.

Usage: rstow [OPTIONS]

Options:
  -n, --simulate  Do not perform any operations that modify the filesystem; merely show what would happen
  -v, --verbose   Send verbose output to standard error describing what rstow is doing
  -d, --unstow    Removes all stowed items
  -f, --force     (Use with care!) Overwrite conflicting files
      --adopt     (Use with care!) Import existing files into stow package
      --dotfiles  Stow packages that start with "dot-" and not "."
  -h, --help      Print help
  -V, --version   Print version
```
