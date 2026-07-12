use std::path::PathBuf;

use lief::elf::{
    Binary,
    builder::Config,
    dynamic::{Entries, Rpath},
};

use crate::{Result, commands::ChangeArgs};

pub fn change_rpath(args: ChangeArgs, path: PathBuf, mut binary: Binary) -> Result<()> {
    match args {
        ChangeArgs::Add { value } => {
            println!("Adding RPath '{value}' to binary.");

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

            binary.remove_dynamic_entry_if(|x| match x {
                Entries::Rpath(rpath) if rpath.rpath() == value => true,
                _ => false,
            });
        },
    }

    println!("Saving binary to {path:?}");
    let config = Config::default();
    binary.write_with_config(path, config);

    Ok(())
}
