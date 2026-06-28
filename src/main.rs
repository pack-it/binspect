use std::fs;

use clap::Parser;
use colored::Colorize;

use crate::{commands::Command, error::Result};

mod binaries;
mod commands;
mod error;
mod utils;

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
    if metadata.is_symlink() {
        println!("The given path is a symlink, following symlink...")
    }

    // Check if the final path is a directory
    if fs::metadata(&command.path)?.is_dir() {
        println!("The given path is a directory");
        return Ok(());
    }

    match lief::Binary::parse(&command.path) {
        Some(lief::Binary::ELF(binary)) => {
            println!("Detected ELF binary!");
            binaries::handle_elf(command, binary)?;
        },
        Some(lief::Binary::MachO(binary)) => {
            println!("Detected MachO binary!");
            binaries::handle_macho(command, binary)?;
        },
        Some(lief::Binary::PE(binary)) => {
            println!("Detected PE binary!");
            binaries::handle_pe(command, binary)?;
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
