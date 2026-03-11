use lief::elf::{Binary, dynamic::Entries};

use crate::Result;

pub fn inspect_elf(binary: Binary) -> Result<()> {
    println!("Found binary for {:?}", binary.header().machine_type()); //TODO: proper to_string

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

    Ok(())
}
