use std::path::PathBuf;

use lief::macho::{Binary, Commands, FatBinary, commands::RPath};

use crate::{Result, commands::ChangeArgs, error::Error};

/// Handles the `RPath` change command for `MachO` binaries.
pub fn change_rpath(args: ChangeArgs, path: PathBuf, mut binary: FatBinary) -> Result<()> {
    for mut binary in binary.iter() {
        match &args {
            ChangeArgs::Add { value, force } => {
                println!("Adding RPath '{value}' to binary.");

                // Check if `RPath` already exists when force is not enabled
                if !force && contains_rpath(&binary, value) {
                    return Err(Error::FieldAlreadyExists {
                        name: "RPath".into(),
                        value: value.clone(),
                    });
                }

                binary.add_command(RPath::new(value));
            },

            ChangeArgs::Change { value, new_value } => {
                println!("Changing RPath '{value}' to '{new_value}' in binary.");

                for mut entry in binary.rpaths() {
                    if entry.path() == *value {
                        entry.set_path(new_value);
                    }
                }
            },

            ChangeArgs::Remove { value } => {
                println!("Removing RPath '{value}' from binary.");

                let mut found = false;
                while let Some((index, _)) = binary.commands().enumerate().find(|(_, cmd)| command_is_rpath(cmd, value)) {
                    binary.remove_command(index as u32);
                    found = true;
                }

                if !found {
                    return Err(Error::FieldNotFound {
                        name: "RPath".into(),
                        value: value.clone(),
                    });
                }
            },
        }
    }

    println!("Saving binary to {path:?}");
    binary.write(&path);

    Ok(())
}

/// Checks if the given binary contains a `RPath` with the given value.
/// Returns true if the value is found, false otherwise.
fn contains_rpath(binary: &Binary, value: &str) -> bool {
    binary.rpaths().any(|x| x.path() == value)
}

/// Checks if the given command is a `RPath` with the given value.
/// Returns true if the command matches, false otherwise.
#[expect(clippy::match_like_matches_macro)]
fn command_is_rpath(command: &Commands<'_>, value: &str) -> bool {
    match command {
        Commands::RPath(rpath) if rpath.path() == *value => true,
        _ => false,
    }
}
