use lief::macho::FatBinary;

use crate::{Command, Result, utils};

pub fn inspect_macho(command: Command, binary: FatBinary) -> Result<()> {
    for binary in binary.iter() {
        println!("Found binary for {:?} {:?}", binary.platform(), binary.header().cpu_type()); //TODO: proper to_string
        println!("Type: {:?}", binary.header().file_type());

        print!("Libraries:");
        let mut found = false;
        for library in binary.libraries() {
            if !found {
                println!();
            }

            let compatibility_version = utils::tuple_version_to_string(library.compatibility_version());
            let current_version = utils::tuple_version_to_string(library.current_version());

            println!(
                "\t{} (compatibility version {}, current version {})",
                library.name(),
                compatibility_version,
                current_version
            );
            found = true;
        }
        if !found {
            println!(" None");
        }

        print!("RPath:");
        let mut found = false;
        for rpath in binary.rpaths() {
            if !found {
                println!();
            }

            println!("\t{}", rpath.path());
            found = true;
        }
        if !found {
            println!(" None");
        }

        if command.flags {
            print!("Flags:");
            let mut found = false;
            for (name, _) in binary.header().flags().iter_names() {
                println!("\t{}", name);
                found = true;
            }
            if !found {
                println!(" None");
            }
        }
    }

    Ok(())
}
