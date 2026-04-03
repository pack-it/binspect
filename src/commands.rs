use std::path::PathBuf;

use clap::{Parser, Subcommand as ClapSubcommand};

#[derive(Parser, Debug)]
#[command(name = "Binspect", version, about)]
pub struct Command {
    /// The path of the binary to inspect.
    pub path: PathBuf,

    /// Flag to include printing of flags.
    #[arg(short, long)]
    pub flags: bool,

    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(ClapSubcommand, Debug, Clone)]
pub enum Subcommand {
    /// Changes the RPath of the binary.
    #[command(subcommand)]
    Rpath(ChangeArgs),

    /// Changes the RunPath of the binary.
    #[command(subcommand)]
    Runpath(ChangeArgs),

    /// Changes the libraries of the binary.
    #[command(subcommand)]
    Library(ChangeArgs),
}

#[derive(ClapSubcommand, Debug, Clone)]
pub enum ChangeArgs {
    /// Adds a value.
    Add {
        value: String,
    },

    /// Changes a value to a new value.
    Change {
        value: String,
        new_value: String,
    },

    /// Removes a value.
    Remove {
        value: String,
    },
}
