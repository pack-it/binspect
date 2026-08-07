use std::path::PathBuf;

use lief::elf::{
    Binary,
    builder::Config,
    dynamic::{Entries, RunPath},
};

use crate::{Result, commands::ChangeArgs, error::Error};

/// Handles the `RunPath` change command for `ELF` binaries.
pub fn change_runpath(args: ChangeArgs, path: PathBuf, mut binary: Binary) -> Result<()> {
    match args {
        ChangeArgs::Add { value, force } => {
            println!("Adding RunPath '{value}' to binary.");

            // Check if `RunPath` already exists when force is not enabled
            if !force && contains_runpath(&binary, &value) {
                return Err(Error::FieldAlreadyExists {
                    name: "RunPath".into(),
                    value,
                });
            }

            binary.add_dynamic_entry(&RunPath::new(&value));
        },

        ChangeArgs::Change { value, new_value } => {
            println!("Changing RunPath '{value}' to '{new_value}' in binary.");

            for entry in binary.dynamic_entries() {
                let mut runpath = match entry {
                    Entries::RunPath(runpath) => runpath,
                    _ => continue,
                };

                if runpath.runpath() == value {
                    runpath.set_runpath(&new_value);
                }
            }
        },

        ChangeArgs::Remove { value } => {
            println!("Removing RunPath '{value}' from binary.");

            binary.remove_dynamic_entry_if(|x| entry_is_runpath(x, &value));
        },
    }

    println!("Saving binary to {}", path.display());
    let config = Config::default();
    binary.write_with_config(path, config);

    Ok(())
}

/// Checks if the given binary contains a `RunPath` with the given value.
/// Returns true if the value is found, false otherwise.
fn contains_runpath(binary: &Binary, value: &str) -> bool {
    binary.dynamic_entries().any(|x| entry_is_runpath(&x, value))
}

/// Checks if the given entry is a `RunPath` with the given value.
/// Returns true if the entry matches, false otherwise.
#[expect(clippy::match_like_matches_macro)]
fn entry_is_runpath(entry: &Entries<'_>, value: &str) -> bool {
    match entry {
        Entries::RunPath(runpath) if runpath.runpath() == value => true,
        _ => false,
    }
}
