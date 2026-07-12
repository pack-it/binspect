use std::path::PathBuf;

use lief::macho::{Commands, FatBinary, builder::Config};

use crate::{Result, commands::ChangeArgs};

pub fn change_library(args: ChangeArgs, path: PathBuf, binary: FatBinary) -> Result<()> {
    for mut binary in binary.iter() {
        match &args {
            ChangeArgs::Add { value } => {
                println!("Adding library '{value}' to binary.");

                binary.add_library(&value);
            },

            ChangeArgs::Change { value, new_value } => {
                println!("Changing library '{value}' to '{new_value}' in binary.");

                match binary.find_library(&value) {
                    Some(mut library) => library.set_name(&new_value),
                    None => {
                        println!("Cannot find library '{value}' in binary.");
                        return Ok(());
                    },
                }
            },

            ChangeArgs::Remove { value } => {
                println!("Removing library '{value}' from binary.");

                let mut found = false;
                while let Some((index, _)) = binary.commands().enumerate().find(|(_, cmd)| match cmd {
                    Commands::Dylib(dylib) if dylib.name() == *value => true,
                    _ => false,
                }) {
                    binary.remove_command(index as u32);
                    found = true;
                }

                if !found {
                    println!("Library '{value}' not found in binary");
                }
            },
        }

        println!("Saving binary to {path:?}");
        let config = Config { linkedit: true };
        binary.write_with_config(&path, config);
    }

    Ok(())
}
