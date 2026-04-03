use anyhow::{bail, Result};
use dialoguer::Select;
use std::path::Path;

use crate::path_store;
use crate::util::{path, search, ui};

pub fn run(arg: &str, all: bool) -> Result<()> {
    let p = Path::new(arg);

    if p.exists() {
        let dir = path::resolve_dir_to_add(p)?;
        let added = path_store::add(&dir)?;
        if added {
            ui::success(&format!("Added to PATH: {}", dir.display()));
            ui::hint_reload();
        } else {
            ui::warn(&format!("Already on PATH: {}", dir.display()));
        }
        return Ok(());
    }

    let on_path = path::find_on_path(arg);
    if !on_path.is_empty() {
        ui::info(&format!("'{}' is already available on PATH:", arg));
        let display: Vec<String> = on_path.iter().map(|p| p.display().to_string()).collect();
        for line in ui::format_tree(&display) {
            eprintln!("{line}");
        }
        return Ok(());
    }

    ui::info(&format!(
        "'{}' not found on PATH, searching filesystem...",
        arg
    ));
    let dirs = search::search_parent_dirs(arg, all)?;

    if dirs.is_empty() {
        bail!("No executables matching '{}' found", arg);
    }

    if dirs.len() == 1 {
        let dir = &dirs[0];
        let added = path_store::add(dir)?;
        if added {
            ui::success(&format!("Added to PATH: {}", dir.display()));
            ui::hint_reload();
        } else {
            ui::warn(&format!("Already on PATH: {}", dir.display()));
        }
        return Ok(());
    }

    let display: Vec<String> = dirs.iter().map(|p| p.display().to_string()).collect();
    let selection = match Select::with_theme(&ui::theme())
        .with_prompt("Select a directory to add to PATH")
        .items(&display)
        .default(0)
        .max_length(10)
        .interact_opt()
    {
        Ok(sel) => sel,
        Err(dialoguer::Error::IO(e)) if e.kind() == std::io::ErrorKind::Interrupted => {
            return Ok(())
        }
        Err(e) => return Err(e.into()),
    };

    let Some(selection) = selection else {
        return Ok(());
    };

    let dir = &dirs[selection];
    let added = path_store::add(dir)?;
    if added {
        ui::success(&format!("Added to PATH: {}", dir.display()));
        ui::hint_reload();
    } else {
        ui::warn(&format!("Already on PATH: {}", dir.display()));
    }
    Ok(())
}
