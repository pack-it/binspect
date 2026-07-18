mod import;
mod inspect;

use lief::pe::Binary;

use crate::{Command, Result, commands::Subcommand};

pub fn handle_pe(command: Command, binary: Binary) -> Result<()> {
    match command.subcommand {
        Some(Subcommand::Rpath(_)) => {
            println!("PE binaries do not support Rpath");
            Ok(())
        },
        Some(Subcommand::Runpath(_)) => {
            println!("PE binaries do not support RunPath");
            Ok(())
        },
        // Libraries are called imports in pe binaries
        Some(Subcommand::Library(args)) => import::change_import(args, command.path, binary),
        None => inspect::inspect_pe(command.inspect_args.unwrap_or_default(), binary),
    }
}
