use std::{fs, path::PathBuf};

use clap::Parser;
use colored::Colorize;

use crate::error::Result;

mod binaries;
mod error;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "Binspect", version, about)]
pub struct Command {
    path: PathBuf,
}

fn main() {
    let command = Command::parse();

    if let Err(e) = handle_command(command) {
        println!("{}: {e}", "ERROR".red().bold());
    }
}

fn handle_command(command: Command) -> Result<()> {
    if !fs::exists(&command.path)? {
        println!("The given path does not exist");
        return Ok(());
    }

    let metadata = fs::symlink_metadata(&command.path)?;
    if metadata.is_dir() {
        println!("The given path is a directory");
        return Ok(());
    }

    if metadata.is_symlink() {
        println!("The given path is a symlink, following symlink...")
    }

    match lief::Binary::parse(command.path) {
        Some(lief::Binary::ELF(binary)) => {
            println!("Detected ELF binary!");
            binaries::inspect_elf(binary)?;
        },
        Some(lief::Binary::MachO(binary)) => {
            println!("Detected MachO binary!");
            binaries::inspect_macho(binary)?;
        },
        Some(lief::Binary::PE(_binary)) => {
            println!("Detected PE binary!");
            println!("This binary type is currently not supported.");
        },
        Some(lief::Binary::COFF(_binary)) => {
            println!("Detected COFF binary!");
            println!("This binary type is currently not supported.");
        },
        None => {
            println!("The given file is not a binary, or the binary is malformed.")
        },
    }

    Ok(())
}
