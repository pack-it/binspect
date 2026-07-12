use std::path::PathBuf;

use lief::elf::{Binary, builder::Config};

use crate::{Result, commands::ChangeArgs};

pub fn change_library(args: ChangeArgs, path: PathBuf, mut binary: Binary) -> Result<()> {
    match args {
        ChangeArgs::Add { value } => {
            println!("Adding library '{value}' to binary.");

            binary.add_library(&value);
        },

        ChangeArgs::Change { value, new_value } => {
            println!("Changing library '{value}' to '{new_value}' in binary.");

            match binary.get_library(&value) {
                Some(mut library) => library.set_name(&new_value),
                None => {
                    println!("Cannot find library '{value}' in binary.");
                    return Ok(());
                },
            }
        },

        ChangeArgs::Remove { value } => {
            println!("Removing library '{value}' from binary.");

            // Check if library exists
            if binary.get_library(&value).is_none() {
                println!("Cannot find library '{value}' in binary.");
                return Ok(());
            }

            binary.remove_library(&value);
        },
    }

    println!("Saving binary to {path:?}");
    let config = Config::default();
    binary.write_with_config(path, config);

    Ok(())
}
