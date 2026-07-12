use std::path::PathBuf;

use lief::macho::{Commands, FatBinary, builder::Config, commands::RPath};

use crate::{Result, commands::ChangeArgs};

pub fn change_rpath(args: ChangeArgs, path: PathBuf, binary: FatBinary) -> Result<()> {
    for mut binary in binary.iter() {
        match &args {
            ChangeArgs::Add { value } => {
                println!("Adding RPath '{value}' to binary.");

                binary.add_command(RPath::new(value));
            },

            ChangeArgs::Change { value, new_value } => {
                println!("Changing RPath '{value}' to '{new_value}' in binary.");

                for mut entry in binary.rpaths() {
                    if entry.path() == *value {
                        entry.set_path(&new_value);
                    }
                }
            },

            ChangeArgs::Remove { value } => {
                println!("Removing RPath '{value}' from binary.");

                let mut found = false;
                while let Some((index, _)) = binary.commands().enumerate().find(|(_, cmd)| match cmd {
                    Commands::RPath(rpath) if rpath.path() == *value => true,
                    _ => false,
                }) {
                    binary.remove_command(index as u32);
                    found = true;
                }

                if !found {
                    println!("RPath '{value}' not found in binary");
                }
            },
        }

        // TODO: save full fat binary instead of the separate binaries
        println!("Saving binary to {path:?}");
        let config = Config { linkedit: true };
        binary.write_with_config(&path, config);
    }

    Ok(())
}
