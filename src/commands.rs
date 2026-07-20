use std::{fmt::Display, path::PathBuf};

use clap::{Args, CommandFactory, Parser, Subcommand as ClapSubcommand};

#[derive(Parser, Debug)]
#[command(name = "Binspect", version, about)]
pub struct Command {
    /// The path of the binary to inspect.
    pub path: PathBuf,

    #[command(flatten)]
    pub inspect_args: Option<InspectOptions>,

    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Args, Debug, Clone)]
pub struct InspectOptions {
    /// Flag to include printing of flags.
    #[arg(long)]
    pub show_flags: bool,
}

#[derive(ClapSubcommand, Debug, Clone)]
pub enum Subcommand {
    /// Changes the RPath of the binary.
    #[command(subcommand)]
    Rpath(ChangeArgs),

    /// Changes the RunPath of the binary.
    #[command(subcommand)]
    Runpath(ChangeArgs),

    /// Changes the libraries of the binary. (In PE binaries these are normally called 'imports')
    #[command(subcommand)]
    Library(ChangeArgs),
}

#[derive(ClapSubcommand, Debug, Clone)]
pub enum ChangeArgs {
    /// Adds a value.
    Add {
        /// The value of the new entry.
        value: String,

        // Flag to force changing, even when checks forbid changes.
        #[arg(short, long)]
        force: bool,
    },

    /// Changes a value to a new value.
    Change {
        /// The old value of the entry to change.
        value: String,

        /// The new value of the entry.
        new_value: String,
    },

    /// Removes a value.
    Remove {
        /// The value of the entry to remove.
        value: String,
    },
}

impl Default for InspectOptions {
    fn default() -> Self {
        Self { show_flags: false }
    }
}

impl Display for Subcommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subcommand::Rpath(_) => write!(f, "rpath"),
            Subcommand::Runpath(_) => write!(f, "runpath"),
            Subcommand::Library(_) => write!(f, "library"),
        }
    }
}

impl Command {
    /// Reads the command from the current arguments.
    pub fn read() -> Self {
        let command = Self::parse();

        if let Some(subcommand) = &command.subcommand
            && command.inspect_args.is_some()
        {
            let mut error = clap::Error::new(clap::error::ErrorKind::ArgumentConflict).with_cmd(&Command::command());
            error.insert(
                clap::error::ContextKind::InvalidSubcommand,
                clap::error::ContextValue::String(subcommand.to_string()),
            );
            error.insert(
                clap::error::ContextKind::PriorArg,
                clap::error::ContextValue::String("--show-flags".into()),
            );
            error.exit();
        }

        command
    }
}
