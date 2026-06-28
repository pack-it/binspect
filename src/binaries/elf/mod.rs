mod inspect;
mod library;
mod rpath;
mod runpath;

use lief::elf::Binary;

use crate::{Command, Result, commands::Subcommand};

pub fn handle_elf(command: Command, binary: Binary) -> Result<()> {
    match command.subcommand {
        Some(Subcommand::Rpath(args)) => rpath::change_rpath(args, command.path, binary),
        Some(Subcommand::Runpath(args)) => runpath::change_runpath(args, command.path, binary),
        Some(Subcommand::Library(args)) => library::change_library(args, command.path, binary),
        None => inspect::inspect_elf(command, binary),
    }
}
