use std::path::PathBuf;

use lief::elf::{
    Binary,
    builder::Config,
    dynamic::{Entries, Rpath, RunPath},
};

use crate::{
    Command, Result,
    commands::{ChangeArgs, Subcommand},
};

pub fn handle_elf(command: Command, binary: Binary) -> Result<()> {
    match command.subcommand {
        Some(Subcommand::Rpath(args)) => change_rpath(args, command.path, binary),
        Some(Subcommand::Runpath(args)) => change_runpath(args, command.path, binary),
        Some(Subcommand::Library(args)) => change_library(args, command.path, binary),
        None => inspect_elf(command, binary),
    }
}

pub fn inspect_elf(command: Command, binary: Binary) -> Result<()> {
    print!("Found binary for {:?}", binary.header().machine_type()); //TODO: proper to_string
    if binary.is_targeting_android() {
        print!(" Android");
    }
    println!();
    println!("Type: {:?}", binary.header().file_type());

    print!("Libraries:");
    let mut found = false;
    for entry in binary.dynamic_entries() {
        let library = match entry {
            Entries::Library(lib) => lib,
            _ => continue,
        };
        if !found {
            println!();
        }

        println!("\t{}", library.name());
        found = true;
    }
    if !found {
        println!(" None");
    }

    print!("RPath:");
    let mut found = false;
    for entry in binary.dynamic_entries() {
        let rpath = match entry {
            Entries::Rpath(rpath) => rpath,
            _ => continue,
        };
        if !found {
            println!();
        }

        println!("\t{}", rpath.rpath());
        found = true;
    }
    if !found {
        println!(" None");
    }

    print!("RunPath:");
    let mut found = false;
    for entry in binary.dynamic_entries() {
        let runpath = match entry {
            Entries::RunPath(runpath) => runpath,
            _ => continue,
        };
        if !found {
            println!();
        }

        println!("\t{}", runpath.runpath());
        found = true;
    }
    if !found {
        println!(" None");
    }

    if command.flags {
        print!("Flags:");
        let mut found = false;
        for entry in binary.dynamic_entries() {
            let flags = match entry {
                Entries::Flags(flags) => flags,
                _ => continue,
            };
            if !found {
                println!();
            }

            for (name, _) in flags.flags().iter_names() {
                println!("\t{}", name);
                found = true;
            }
        }
        if !found {
            println!(" None");
        }
    }

    Ok(())
}

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
            // TODO: check if library exists
            binary.remove_library(&value);
        },
    }

    println!("Saving binary to {path:?}");
    let config = Config::default();
    binary.write_with_config(path, config);

    Ok(())
}
