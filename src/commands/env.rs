use anyhow::Result;

use crate::cli::Shell;
use crate::path_store;
#[cfg(unix)]
use crate::util::shell;

pub fn run(shell_type: &Shell) -> Result<()> {
    #[cfg(windows)]
    let parts: Vec<String> = {
        let effective = path_store::effective_registry_path()?;
        effective
            .split(';')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    };

    #[cfg(unix)]
    let parts: Vec<String> = {
        let path = std::env::var("PATH").unwrap_or_default();
        let managed = path_store::list_managed()?;
        let blocked = path_store::list_blocked().unwrap_or_default();
        shell::compute_env_path(&path, &managed, &blocked)
    };

    match shell_type {
        Shell::Bash | Shell::Zsh => {
            #[cfg(windows)]
            {
                let registry_path = parts.join(";");
                let mut converted = cygpath(&registry_path)?;
                // Preserve paths not in the Windows registry (MSYS, shell config, etc.).
                if let Ok(current) = std::env::var("PATH") {
                    let registry_set: std::collections::HashSet<String> = parts
                        .iter()
                        .map(|p| p.trim_end_matches('\\').to_lowercase())
                        .collect();
                    for entry in current.split(';').filter(|s| !s.is_empty()) {
                        let normalized = entry.trim_end_matches('\\').to_lowercase();
                        if registry_set.contains(&normalized) {
                            continue;
                        }
                        for e in cygpath(entry)? {
                            if !converted.contains(&e) {
                                converted.push(e);
                            }
                        }
                    }
                }
                print!("{}", converted.join(":"));
            }
            #[cfg(unix)]
            print!("{}", parts.join(":"));
        }
        Shell::Fish | Shell::Xonsh | Shell::Nushell => {
            for p in &parts {
                println!("{}", p);
            }
        }
        Shell::Pwsh => {
            let sep = if cfg!(windows) { ";" } else { ":" };
            print!("{}", parts.join(sep));
        }
    }

    Ok(())
}

#[cfg(windows)]
fn cygpath(path_list: &str) -> Result<Vec<String>> {
    let output = std::process::Command::new("cygpath")
        .args(["-u", "-p", path_list])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run cygpath: {e}"))?;
    if !output.status.success() {
        anyhow::bail!(
            "cygpath failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .split(':')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect())
}
