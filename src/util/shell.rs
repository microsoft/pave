#[cfg(unix)]
use std::collections::VecDeque;
#[cfg(unix)]
use std::path::PathBuf;

use crate::cli::Shell;

pub fn detect_shell() -> Option<Shell> {
    if std::env::var("NU_VERSION").is_ok() {
        return Some(Shell::Nushell);
    }
    if std::env::var("XONSHRC").is_ok() {
        return Some(Shell::Xonsh);
    }
    if std::env::var("FISH_VERSION").is_ok() {
        return Some(Shell::Fish);
    }
    if std::env::var("ZSH_VERSION").is_ok() {
        return Some(Shell::Zsh);
    }
    if std::env::var("BASH_VERSION").is_ok() {
        return Some(Shell::Bash);
    }
    #[cfg(unix)]
    if std::env::var("PSModulePath").is_ok() {
        return Some(Shell::Pwsh);
    }

    if let Ok(shell_env) = std::env::var("SHELL") {
        let name = std::path::Path::new(&shell_env)
            .file_name()
            .and_then(|n| n.to_str());
        match name {
            Some("bash") => return Some(Shell::Bash),
            Some("zsh") => return Some(Shell::Zsh),
            Some("fish") => return Some(Shell::Fish),
            Some("nu") => return Some(Shell::Nushell),
            Some("xonsh") => return Some(Shell::Xonsh),
            Some("pwsh") => return Some(Shell::Pwsh),
            _ => {}
        }
    }

    #[cfg(windows)]
    if let Some(name) = parent_process_name() {
        let name_lower = name.to_lowercase();
        if name_lower == "bash.exe" {
            return Some(Shell::Bash);
        } else if name_lower == "pwsh.exe" || name_lower == "powershell.exe" {
            return Some(Shell::Pwsh);
        } else if name_lower == "nu.exe" {
            return Some(Shell::Nushell);
        } else if name_lower == "fish.exe" || name_lower == "fish" {
            return Some(Shell::Fish);
        } else if name_lower == "xonsh.exe" {
            return Some(Shell::Xonsh);
        } else if name_lower == "zsh.exe" {
            return Some(Shell::Zsh);
        }
    }

    None
}

#[cfg(windows)]
fn parent_process_name() -> Option<String> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    };

    let current_pid = std::process::id();
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
        return None;
    }

    let mut entry: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

    let mut parent_pid = None;
    if unsafe { Process32FirstW(snapshot, &mut entry) } != 0 {
        loop {
            if entry.th32ProcessID == current_pid {
                parent_pid = Some(entry.th32ParentProcessID);
                break;
            }
            if unsafe { Process32NextW(snapshot, &mut entry) } == 0 {
                break;
            }
        }
    }

    let result = parent_pid.and_then(|ppid| {
        let mut entry2: PROCESSENTRY32W = unsafe { std::mem::zeroed() };
        entry2.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if unsafe { Process32FirstW(snapshot, &mut entry2) } != 0 {
            loop {
                if entry2.th32ProcessID == ppid {
                    let len = entry2
                        .szExeFile
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry2.szExeFile.len());
                    return Some(String::from_utf16_lossy(&entry2.szExeFile[..len]));
                }
                if unsafe { Process32NextW(snapshot, &mut entry2) } == 0 {
                    break;
                }
            }
        }
        None
    });

    unsafe { CloseHandle(snapshot) };
    result
}

#[cfg(unix)]
pub fn compute_env_path(
    current_path: &str,
    managed: &[PathBuf],
    blocked: &[PathBuf],
) -> Vec<String> {
    let mut dirs: VecDeque<String> = current_path
        .split(':')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    for b in blocked {
        let b_str = b.display().to_string();
        dirs.retain(|d| d != &b_str);
    }

    for p in managed.iter().rev() {
        let p_str = p.display().to_string();
        dirs.retain(|d| d != &p_str);
        dirs.push_front(p_str);
    }

    dirs.into_iter().collect()
}
