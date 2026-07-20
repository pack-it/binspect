use lief::macho::{Binary, FatBinary};

use crate::{
    Result,
    commands::InspectOptions,
    utils::{self, TupleVersion},
};

/// Handles the inspect command for `MachO` binaries.
pub fn inspect_macho(args: InspectOptions, binary: FatBinary) -> Result<()> {
    for binary in binary.iter() {
        println!("Found binary for {:?} {:?}", binary.platform(), binary.header().cpu_type()); //TODO: proper to_string
        println!("Type: {:?}", binary.header().file_type());

        print!("Minimum OS version: ");
        match get_minimum_os_version(&binary) {
            Some((sdk, minos)) => println!(
                "{} (sdk {})",
                utils::tuple_version_to_string(minos),
                utils::tuple_version_to_string(sdk)
            ),
            None => println!("Not found"),
        };

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

        if args.show_flags {
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

/// Gets the minimum os version required for running the binary.
/// Returns None if a minimum version cannot be found.
fn get_minimum_os_version(binary: &Binary) -> Option<(TupleVersion, TupleVersion)> {
    if let Some(version) = binary.version_min() {
        return Some((version.sdk(), version.version()));
    }

    if let Some(version) = binary.build_version() {
        return Some((version.sdk(), version.minos()));
    }

    None
}
