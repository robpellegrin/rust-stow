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
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(arg_required_else_help = false)]
#[command(
    version = "0.2.3",
    author = "Robert Pellegrin",
    about = "A simple, fast tool for dotfile managment."
)]

pub struct Args {
    /// Do not perform any operations that modify the filesystem; merely show what would happen.
    #[arg(short = 'n', long, action = ArgAction::SetTrue)]
    pub simulate: bool,

    /// Send verbose output to standard error describing what rstow is doing.
    #[arg(short = 'v', long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Removes all stowed items.
    #[arg(short = 'd', long, action = ArgAction::SetTrue)]
    pub unstow: bool,

    /// (Use with care!) Import existing files into stow package
    #[arg(long, action = ArgAction::SetTrue)]
    pub adopt: bool,

    /// (Use with care!) Overwrite conflicting files
    #[arg(short='f',long,  action = ArgAction::SetTrue)]
    pub force: bool,

    /// Stow packages that start with "dot-" and not ".".
    #[arg(long, action = ArgAction::SetTrue)]
    pub dotfiles: bool,
}
