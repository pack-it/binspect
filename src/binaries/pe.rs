use lief::{generic::Section, pe::Binary};
use pe_loader_lib::rpath_section::RPathSection;

use crate::{Command, Result};

pub fn inspect_pe(_command: Command, binary: Binary) -> Result<()> {
    println!("Found binary for {:?}", binary.header().machine()); //TODO: proper to_string

    print!("Imports:");
    let mut found = false;
    for import in binary.imports() {
        if !found {
            println!();
        }

        println!("\t{}", import.name());

        found = true;
    }
    for import in binary.delay_imports() {
        if !found {
            println!();
        }

        println!("\t{} (delay)", import.name());

        found = true;
    }
    if !found {
        println!(" None");
    }

    print!("RPath:");
    let mut found = false;
    for section in binary.sections() {
        if section.name() != ".rpath" {
            continue;
        }
        let rpath: RPathSection = match section.try_into() {
            Ok(rpath) => rpath,
            Err(_) => {
                println!("WARNING: Found invalid .rpath section.");
                continue;
            },
        };

        for entry in rpath.entries {
            if !found {
                println!();
            }

            println!("\t{}", entry.path);
            found = true;
        }
    }
    if !found {
        println!(" None");
    }

    Ok(())
}
