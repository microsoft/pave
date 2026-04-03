use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn path_dirs() -> impl Iterator<Item = String> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ';' } else { ':' };
    path_var
        .split(separator)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn executable_candidates(name_lower: &str) -> Vec<String> {
    #[allow(unused_mut)]
    let mut candidates = vec![name_lower.to_string()];

    #[cfg(windows)]
    {
        let has_ext = Path::new(name_lower).extension().is_some_and(|ext| {
            matches!(
                ext.to_string_lossy().as_ref(),
                "exe" | "cmd" | "bat" | "com" | "ps1"
            )
        });
        if !has_ext {
            for ext in &["exe", "cmd", "bat", "com", "ps1"] {
                candidates.push(format!("{}.{}", name_lower, ext));
            }
        }
    }

    candidates
}

pub fn find_on_path(name: &str) -> Vec<PathBuf> {
    let name_lower = name.to_lowercase();
    let candidates = executable_candidates(&name_lower);
    let mut results = Vec::new();

    for dir in path_dirs() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let is_file = entry.file_type().is_ok_and(|ft| ft.is_file());
            if !is_file {
                continue;
            }
            let fname = entry.file_name().to_string_lossy().to_lowercase();
            if candidates.iter().any(|c| c == &fname) {
                results.push(entry.path());
            }
        }
    }
    results
}

pub fn search_executables(query: &str) -> Vec<PathBuf> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for dir in path_dirs() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy().to_lowercase();
            if !name.contains(&query_lower) {
                continue;
            }

            let path = entry.path();
            if !is_executable(&path) {
                continue;
            }

            results.push(path);
        }
    }
    results
}

pub fn resolve_dir_to_add(path: &Path) -> Result<PathBuf> {
    let abs = dunce::canonicalize(path)
        .map_err(|_| anyhow::anyhow!("Cannot resolve path: {}", path.display()))?;

    if abs.is_file() {
        abs.parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| anyhow::anyhow!("File has no parent directory: {}", abs.display()))
    } else {
        Ok(abs)
    }
}

#[cfg(unix)]
pub fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.is_file()
        && path
            .metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

#[cfg(windows)]
pub fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    let Some(ext) = path.extension() else {
        return false;
    };
    let ext = ext.to_string_lossy().to_lowercase();
    matches!(ext.as_str(), "exe" | "cmd" | "bat" | "com" | "ps1")
}

pub fn drive_root() -> PathBuf {
    #[allow(unused_variables)]
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    #[cfg(windows)]
    {
        if let Some(prefix) = cwd.components().next() {
            let mut root = PathBuf::from(prefix.as_os_str());
            root.push(std::path::MAIN_SEPARATOR_STR);
            return root;
        }
    }
    PathBuf::from("/")
}
