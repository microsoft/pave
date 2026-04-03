use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "pave",
    about = "A cross-platform CLI tool for managing the PATH"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a directory or executable to the PATH
    Add {
        /// A directory path, file path, or executable name to add to the PATH
        path: String,

        /// Search hidden and ignored folders too
        #[arg(long)]
        all: bool,
    },

    /// Remove a directory from the PATH
    Remove {
        /// The path to remove. If omitted, shows an interactive picker.
        path: Option<String>,
    },

    /// List all directories on the PATH
    List,

    /// Search for executables matching a query across PATH directories
    Search {
        /// The name or partial name to search for
        query: String,
    },

    /// Output the current PATH value for a given shell
    Env {
        /// The shell to output the PATH for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Xonsh,
    Nushell,
    Pwsh,
}
