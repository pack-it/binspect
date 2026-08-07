use std::path::PathBuf;

use lief::elf::{Binary, builder::Config};

use crate::{Result, commands::ChangeArgs, error::Error};

/// Handles the library change command for `ELF` binaries.
pub fn change_library(args: ChangeArgs, path: PathBuf, mut binary: Binary) -> Result<()> {
    match args {
        ChangeArgs::Add { value, force } => {
            println!("Adding library '{value}' to binary.");

            // Check if library already exists when force is not enabled
            if !force && binary.has_library(&value) {
                return Err(Error::FieldAlreadyExists {
                    name: "library".into(),
                    value,
                });
            }

            binary.add_library(&value);
        },

        ChangeArgs::Change { value, new_value } => {
            println!("Changing library '{value}' to '{new_value}' in binary.");

            match binary.get_library(&value) {
                Some(mut library) => library.set_name(&new_value),
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

            // Check if library exists
            if binary.get_library(&value).is_none() {
                return Err(Error::FieldNotFound {
                    name: "library".into(),
                    value: value.clone(),
                });
            }

            binary.remove_library(&value);
        },
    }

    println!("Saving binary to {}", path.display());
    let config = Config::default();
    binary.write_with_config(path, config);

    Ok(())
}
