mod inspect;

use lief::pe::Binary;

use crate::{Command, Result, commands::Subcommand};

pub fn handle_pe(command: Command, binary: Binary) -> Result<()> {
    match command.subcommand {
        Some(Subcommand::Rpath(args)) => todo!(),
        Some(Subcommand::Runpath(args)) => todo!(),
        Some(Subcommand::Library(args)) => todo!(),
        None => inspect::inspect_pe(command, binary),
    }
}
