use anyhow::{bail, Result};
use dialoguer::Select;
use std::path::Path;

use crate::path_store;
use crate::util::{path, ui};

pub fn run(path_arg: Option<&str>) -> Result<()> {
    match path_arg {
        Some(p) => remove_directly(p),
        None => remove_interactive(),
    }
}

fn remove_directly(path_str: &str) -> Result<()> {
    let p = Path::new(path_str);

    if path_store::remove(p)? {
        ui::success(&format!("Removed from PATH: {}", p.display()));
        ui::hint_reload();
        return Ok(());
    }

    if path_store::is_in_env_path(p) {
        if cfg!(windows) {
            bail!(
                "'{}' is a system PATH entry and cannot be removed without admin rights",
                p.display()
            );
        }
        path_store::block(p)?;
        ui::success(&format!("Blocked from PATH: {}", p.display()));
        ui::hint("This path comes from your shell config, not pave.");
        ui::hint_reload();
        return Ok(());
    }

    bail!("'{}' is not on PATH", p.display());
}

fn remove_interactive() -> Result<()> {
    let managed = path_store::list_managed()?;
    let env_dirs: Vec<String> = path::path_dirs().collect();

    let mut entries: Vec<(String, bool)> = Vec::new();
    for p in &managed {
        entries.push((p.display().to_string(), true));
    }
    if !cfg!(windows) {
        let mut canonical_set: Vec<std::path::PathBuf> = entries
            .iter()
            .map(|(d, _)| dunce::canonicalize(d).unwrap_or_default())
            .collect();

        for dir in &env_dirs {
            let canon_dir = dunce::canonicalize(dir).unwrap_or_default();
            let already = canonical_set.contains(&canon_dir);
            if !already {
                canonical_set.push(canon_dir);
                entries.push((dir.clone(), false));
            }
        }
    }

    if entries.is_empty() {
        ui::warn("No PATH entries to remove.");
        return Ok(());
    }

    let display: Vec<String> = entries
        .iter()
        .map(|(p, is_managed)| {
            if *is_managed {
                p.clone()
            } else {
                format!("{p} (env)")
            }
        })
        .collect();

    let selection = match Select::with_theme(&ui::theme())
        .with_prompt("Select an entry to remove from PATH")
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

    let Some(idx) = selection else {
        return Ok(());
    };

    let (entry_path, is_managed) = &entries[idx];
    let p = Path::new(entry_path);
    if *is_managed {
        path_store::remove(p)?;
        ui::success(&format!("Removed: {}", p.display()));
    } else {
        path_store::block(p)?;
        ui::success(&format!("Blocked: {}", p.display()));
    }

    ui::hint_reload();
    Ok(())
}
