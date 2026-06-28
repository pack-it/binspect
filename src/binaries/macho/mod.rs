mod inspect;

use lief::macho::FatBinary;

use crate::{Command, Result, commands::Subcommand};

pub fn handle_macho(command: Command, binary: FatBinary) -> Result<()> {
    match command.subcommand {
        Some(Subcommand::Rpath(args)) => todo!(),
        Some(Subcommand::Runpath(args)) => todo!(),
        Some(Subcommand::Library(args)) => todo!(),
        None => inspect::inspect_macho(command, binary),
    }
}
