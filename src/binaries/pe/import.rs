use std::path::PathBuf;

use lief::pe::{Binary, builder::Config};

use crate::{Result, commands::ChangeArgs, error::Error};

pub fn change_import(args: ChangeArgs, path: PathBuf, mut binary: Binary) -> Result<()> {
    match args {
        ChangeArgs::Add { value, force } => {
            println!("Adding import '{value}' to binary.");

            binary.add_import(&value);
        },

        ChangeArgs::Change { value, new_value } => {
            println!("Changing import '{value}' to '{new_value}' in binary.");

            match binary.import_by_name(&value) {
                Some(mut import) => import.set_name(&new_value),
                None => {
                    return Err(Error::FieldNotFound {
                        name: "import".into(),
                        value,
                    });
                },
            }
        },

        ChangeArgs::Remove { value } => {
            println!("Removing import '{value}' from binary.");

            // Check if import exists
            if binary.import_by_name(&value).is_none() {
                return Err(Error::FieldNotFound {
                    name: "import".into(),
                    value,
                });
            }

            binary.remove_import(&value);
        },
    }

    println!("Saving binary to {path:?}");
    let mut config = Config::default();
    config.imports = true;
    binary.write_with_config(path, config);

    Ok(())
}
