use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Binspect", version, about)]
pub struct Command {
    /// The path of the binary to inspect.
    pub path: PathBuf,

    /// Flag to include printing of flags
    #[arg(short, long)]
    pub flags: bool,
}
