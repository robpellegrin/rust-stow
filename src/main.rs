mod args;
use args::Args;
use clap::Parser;

// Interm program name.
const PROGRAM_NAME: &str = "rstow";

fn main() {
    let args = Args::parse();
    println!("Welcome to {}", PROGRAM_NAME);
}
