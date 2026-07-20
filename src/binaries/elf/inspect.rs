use lief::elf::{Binary, dynamic::Entries};

use crate::{Result, commands::InspectOptions};

pub fn inspect_elf(args: InspectOptions, binary: Binary) -> Result<()> {
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

    if args.show_flags {
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
