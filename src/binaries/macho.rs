use lief::macho::FatBinary;

use crate::{Result, utils};

pub fn inspect_macho(binary: FatBinary) -> Result<()> {
    for binary in binary.iter() {
        println!("Found binary for {:?} {:?}", binary.platform(), binary.header().cpu_type()); //TODO: proper to_string

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
    }

    Ok(())
}
