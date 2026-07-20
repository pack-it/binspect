use std::path::PathBuf;

use lief::elf::{
    Binary,
    builder::Config,
    dynamic::{Entries, Rpath},
};

use crate::{Result, commands::ChangeArgs, error::Error};

/// Handles the `RPath` change command for `ELF` binaries.
pub fn change_rpath(args: ChangeArgs, path: PathBuf, mut binary: Binary) -> Result<()> {
    match args {
        ChangeArgs::Add { value, force } => {
            println!("Adding RPath '{value}' to binary.");

            // Check if `RPath` already exists when force is not enabled
            if !force && contains_rpath(&binary, &value) {
                return Err(Error::FieldAlreadyExists {
                    name: "RPath".into(),
                    value,
                });
            }

            binary.add_dynamic_entry(&Rpath::new(&value));
        },

        ChangeArgs::Change { value, new_value } => {
            println!("Changing RPath '{value}' to '{new_value}' in binary.");

            for entry in binary.dynamic_entries() {
                let mut rpath = match entry {
                    Entries::Rpath(rpath) => rpath,
                    _ => continue,
                };

                if rpath.rpath() == value {
                    rpath.set_rpath(&new_value);
                }
            }
        },

        ChangeArgs::Remove { value } => {
            println!("Removing RPath '{value}' from binary.");

            binary.remove_dynamic_entry_if(|x| entry_is_rpath(x, &value));
        },
    }

    println!("Saving binary to {path:?}");
    let config = Config::default();
    binary.write_with_config(path, config);

    Ok(())
}

/// Checks if the given binary contains a `RPath` with the given value.
/// Returns true if the value is found, false otherwise.
fn contains_rpath(binary: &Binary, value: &str) -> bool {
    binary.dynamic_entries().any(|x| entry_is_rpath(&x, value))
}

/// Checks if the given entry is a `RPath` with the given value.
/// Returns true if the entry matches, false otherwise.
#[expect(clippy::match_like_matches_macro)]
fn entry_is_rpath(entry: &Entries<'_>, value: &str) -> bool {
    match entry {
        Entries::Rpath(rpath) if rpath.rpath() == value => true,
        _ => false,
    }
}
