mod inspect;
mod library;
mod rpath;

use lief::macho::FatBinary;

use crate::{Command, Result, commands::Subcommand};

pub fn handle_macho(command: Command, binary: FatBinary) -> Result<()> {
    match command.subcommand {
        Some(Subcommand::Rpath(args)) => rpath::change_rpath(args, command.path, binary),
        Some(Subcommand::Runpath(_)) => {
            println!("MachO binaries do not support RunPath");
            Ok(())
        },
        Some(Subcommand::Library(args)) => library::change_library(args, command.path, binary),
        None => inspect::inspect_macho(command.inspect_args.unwrap_or_default(), binary),
    }
}
