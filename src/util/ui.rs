use console::Style;
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::Shell;
use crate::util::shell;

pub fn theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_prefix: console::style("?".to_string()).cyan().bold(),
        active_item_prefix: console::style("›".to_string()).cyan().bold(),
        active_item_style: Style::new().cyan(),
        checked_item_prefix: console::style("✓".to_string()).green().bold(),
        unchecked_item_prefix: console::style("○".to_string()).dim(),
        ..ColorfulTheme::default()
    }
}

pub fn success(msg: &str) {
    let style = Style::new().green().bold();
    eprintln!("{} {msg}", style.apply_to("✓"));
}

pub fn warn(msg: &str) {
    let style = Style::new().yellow().bold();
    eprintln!("{} {msg}", style.apply_to("!"));
}

pub fn info(msg: &str) {
    let style = Style::new().cyan().bold();
    eprintln!("{} {msg}", style.apply_to("●"));
}

pub fn hint(msg: &str) {
    let style = Style::new().dim();
    eprintln!("  {}", style.apply_to(msg));
}

#[allow(unused)]
pub fn error(msg: &str) {
    let style = Style::new().red().bold();
    eprintln!("{} {msg}", style.apply_to("✗"));
}

pub fn format_tree(items: &[impl std::fmt::Display]) -> Vec<String> {
    let dim = Style::new().dim();
    items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let connector = if i == items.len() - 1 {
                "╰─"
            } else {
                "├─"
            };
            format!("  {} {}", dim.apply_to(connector), item)
        })
        .collect()
}

pub fn hint_reload() {
    #[cfg(unix)]
    {
        match shell::detect_shell() {
            Some(Shell::Bash) => {
                hint("Restart your shell or re-run `export PATH=\"$(pave env bash)\"` to apply.");
            }
            Some(Shell::Zsh) => {
                hint("Restart your shell or re-run `export PATH=\"$(pave env zsh)\"` to apply.");
            }
            Some(Shell::Fish) => {
                hint("Restart your shell or re-run `set -gx PATH (pave env fish)` to apply.");
            }
            Some(Shell::Pwsh) => {
                hint("Restart your shell or re-run `$env:PATH = (pave env pwsh)` to apply.");
            }
            Some(Shell::Xonsh) => {
                hint("Restart your shell or re-run `$PATH = $(pave env xonsh).strip().split('\\n')` to apply.");
            }
            Some(Shell::Nushell) => {
                hint("Restart your shell or re-run `$env.PATH = (pave env nushell | lines)` to apply.");
            }
            None => {
                hint("Open a new terminal window for the updated PATH to take effect.");
            }
        }
    }
    #[cfg(windows)]
    {
        match shell::detect_shell() {
            Some(Shell::Bash) => {
                hint("Run `export PATH=\"$(pave env bash)\"` to apply in this session, or open a new terminal.");
            }
            Some(Shell::Zsh) => {
                hint("Run `export PATH=\"$(pave env zsh)\"` to apply in this session, or open a new terminal.");
            }
            Some(Shell::Fish) => {
                hint("Run `set -gx PATH (pave env fish)` to apply in this session, or open a new terminal.");
            }
            Some(Shell::Pwsh) => {
                hint("Run `$env:PATH = (pave env pwsh)` to apply in this session, or open a new terminal.");
            }
            Some(Shell::Xonsh) => {
                hint("Run `$PATH = $(pave env xonsh).strip().split('\\n')` to apply in this session, or open a new terminal.");
            }
            Some(Shell::Nushell) | None => {
                hint("Open a new terminal window for the updated PATH to take effect.");
            }
        }
    }
}

pub fn new_search_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    spinner.set_message("Searching...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner
}
