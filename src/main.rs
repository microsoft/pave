mod cli;
mod commands;
mod path_store;
mod util;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use console::Term;

fn main() -> anyhow::Result<()> {
    ctrlc::set_handler(move || {
        let _ = Term::stderr().show_cursor();
        std::process::exit(130);
    })?;

    let cli = Cli::parse();

    let Some(command) = &cli.command else {
        Cli::command().print_help()?;
        return Ok(());
    };

    match command {
        Commands::Add { path, all } => commands::add::run(path, *all)?,
        Commands::Remove { path } => commands::remove::run(path.as_deref())?,
        Commands::List => commands::list::run()?,
        Commands::Search { query } => commands::search::run(query)?,
        Commands::Env { shell } => commands::env::run(shell)?,
    }

    Ok(())
}
