mod import;
mod inspect;

use lief::pe::Binary;

use crate::{Command, Result, commands::Subcommand, macros::error};

/// Handles a command for a `PE` binary.
pub fn handle_pe(command: Command, binary: Binary) -> Result<()> {
    match command.subcommand {
        Some(Subcommand::Rpath(_)) => {
            error!("PE binaries do not support Rpath");
            Ok(())
        },
        Some(Subcommand::Runpath(_)) => {
            error!("PE binaries do not support RunPath");
            Ok(())
        },
        // Libraries are called imports in `PE` binaries
        Some(Subcommand::Library(args)) => import::change_import(args, command.path, binary),
        Some(Subcommand::Sign) => {
            error!("PE binaries do not need signing");
            Ok(())
        },
        None => inspect::inspect_pe(command.inspect_args.unwrap_or_default(), binary),
    }
}
