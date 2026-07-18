use lief::pe::Binary;

use crate::{Result, commands::InspectOptions};

pub fn inspect_pe(_args: InspectOptions, binary: Binary) -> Result<()> {
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

    Ok(())
}
