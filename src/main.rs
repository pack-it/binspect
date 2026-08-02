#![warn(clippy::doc_markdown, clippy::inconsistent_struct_constructor, clippy::derive_partial_eq_without_eq)]
#![warn(clippy::cargo, clippy::perf, clippy::complexity)]
#![allow(clippy::enum_variant_names)]
use std::{fs, path::PathBuf};

use crate::{commands::Command, error::Result, macros::error};

mod binaries;
mod commands;
mod error;
mod macros;
mod utils;

fn main() {
    let command = Command::read();

    if let Err(e) = handle_command(command) {
        error!("{e}");
    }
}

/// Handles the given command.
fn handle_command(command: Command) -> Result<()> {
    if !fs::exists(&command.path)? {
        error!("The given path does not exist");
        return Ok(());
    }

    show_symlink_traversal(&command.path)?;
    println!();

    // Check if the final path is a directory
    if fs::metadata(&command.path)?.is_dir() {
        error!("The given path is a directory");
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
            error!("This binary type is currently not supported.");
        },
        None => {
            error!("The given file is not a binary, or the binary is malformed.")
        },
    }

    Ok(())
}

/// Shows the symlink traversal to the user.
fn show_symlink_traversal(path: &PathBuf) -> Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_symlink() {
        return Ok(());
    }

    println!("The given path '{}' is a symlink, following symlink...", path.display());

    // Get the symlink destination
    let symlink_destination = fs::read_link(path)?;

    // Get the absolute path of the symlink
    let absolute_path = match symlink_destination.is_absolute() {
        true => symlink_destination,
        false => match path.parent() {
            Some(parent) => parent.join(symlink_destination),
            None => return Ok(()),
        },
    };

    // Show the user where the symlink resolved to
    println!("Symlink resolved to: {}", absolute_path.display());

    // Traverse further if necessary
    show_symlink_traversal(&absolute_path)?;

    Ok(())
}
