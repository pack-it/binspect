use std::path::PathBuf;

use lief::macho::{Commands, FatBinary};

use crate::{Result, commands::ChangeArgs, error::Error};

/// Handles the library change command for `MachO` binaries.
pub fn change_library(args: ChangeArgs, path: PathBuf, mut binary: FatBinary) -> Result<()> {
    for mut binary in binary.iter() {
        match &args {
            ChangeArgs::Add { value, force } => {
                println!("Adding library '{value}' to binary.");

                // Check if library already exists when force is not enabled
                if !force && binary.find_library(value).is_some() {
                    return Err(Error::FieldAlreadyExists {
                        name: "library".into(),
                        value: value.clone(),
                    });
                }

                binary.add_library(value);
            },

            ChangeArgs::Change { value, new_value } => {
                println!("Changing library '{value}' to '{new_value}' in binary.");

                match binary.find_library(value) {
                    Some(mut library) => library.set_name(new_value),
                    None => {
                        return Err(Error::FieldNotFound {
                            name: "library".into(),
                            value: value.clone(),
                        });
                    },
                }
            },

            ChangeArgs::Remove { value } => {
                println!("Removing library '{value}' from binary.");

                let mut found = false;
                while let Some((index, _)) = binary.commands().enumerate().find(|(_, cmd)| command_is_library(cmd, value)) {
                    binary.remove_command(index as u32);
                    found = true;
                }

                if !found {
                    return Err(Error::FieldNotFound {
                        name: "library".into(),
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

/// Checks if the given command is a library with the given value.
/// Returns true if the command matches, false otherwise.
#[expect(clippy::match_like_matches_macro)]
fn command_is_library(command: &Commands<'_>, value: &str) -> bool {
    match command {
        Commands::Dylib(dylib) if dylib.name() == value => true,
        _ => false,
    }
}
