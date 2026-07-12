use std::path::PathBuf;

use lief::elf::{
    Binary,
    builder::Config,
    dynamic::{Entries, RunPath},
};

use crate::{Result, commands::ChangeArgs};

pub fn change_runpath(args: ChangeArgs, path: PathBuf, mut binary: Binary) -> Result<()> {
    match args {
        ChangeArgs::Add { value } => {
            println!("Adding RunPath '{value}' to binary.");

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

            binary.remove_dynamic_entry_if(|x| match x {
                Entries::RunPath(runpath) if runpath.runpath() == value => true,
                _ => false,
            });
        },
    }

    println!("Saving binary to {path:?}");
    let config = Config::default();
    binary.write_with_config(path, config);

    Ok(())
}
