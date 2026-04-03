use anyhow::Result;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use super::path::{drive_root, executable_candidates};
use super::ui::new_search_spinner;

pub fn search_filesystem_files(candidates: &[String], all: bool) -> Result<Vec<PathBuf>> {
    let root = drive_root();

    let files: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));
    let scanned: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));

    let spinner = new_search_spinner();

    let mut builder = WalkBuilder::new(&root);
    builder.threads(std::thread::available_parallelism().map_or(4, |n| n.get()));
    if all {
        builder
            .hidden(false)
            .ignore(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false);
    } else {
        builder.overrides(build_overrides(&root)?);
    }
    let walker = builder.build_parallel();

    walker.run(|| {
        let candidates = candidates.to_vec();
        let files = Arc::clone(&files);
        let scanned = Arc::clone(&scanned);
        let spinner = spinner.clone();
        Box::new(move |entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return ignore::WalkState::Continue,
            };

            #[cfg(windows)]
            if should_skip_cloud(&entry) {
                return if entry.file_type().is_some_and(|ft| ft.is_dir()) {
                    ignore::WalkState::Skip
                } else {
                    ignore::WalkState::Continue
                };
            }

            let count = scanned.fetch_add(1, Ordering::Relaxed) + 1;
            if count.is_multiple_of(1000) {
                spinner.set_message(format!("Searching... {count} entries scanned"));
            }

            if !entry.path().is_file() {
                return ignore::WalkState::Continue;
            }
            if let Some(file_name) = entry.path().file_name() {
                let fname = file_name.to_string_lossy().to_lowercase();
                if candidates.iter().any(|c| c == &fname) {
                    if let Ok(abs) = dunce::canonicalize(entry.path()) {
                        files.lock().unwrap().insert(abs);
                    }
                }
            }
            ignore::WalkState::Continue
        })
    });

    let total = scanned.load(Ordering::Relaxed);
    spinner.finish_and_clear();
    super::ui::info(&format!("Searched {total} entries."));

    let mut files: Vec<PathBuf> = Arc::try_unwrap(files)
        .unwrap()
        .into_inner()
        .unwrap()
        .into_iter()
        .collect();
    files.sort();
    Ok(files)
}

pub fn search_parent_dirs(name: &str, all: bool) -> Result<Vec<PathBuf>> {
    let name_lower = name.to_lowercase();
    let candidates = executable_candidates(&name_lower);
    let files = search_filesystem_files(&candidates, all)?;
    let mut seen = HashSet::new();
    let mut dirs: Vec<PathBuf> = Vec::new();
    for f in &files {
        if let Some(parent) = f.parent() {
            let parent = parent.to_path_buf();
            if seen.insert(parent.clone()) {
                dirs.push(parent);
            }
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn build_overrides(root: &Path) -> Result<ignore::overrides::Override> {
    let mut overrides = OverrideBuilder::new(root);
    for dir in &[
        "node_modules",
        ".git",
        "__pycache__",
        ".cache",
        "site-packages",
    ] {
        overrides.add(&format!("!**/{dir}/"))?;
    }
    #[cfg(windows)]
    for dir in &["Windows/WinSxS", "Windows/assembly", "Windows/servicing"] {
        overrides.add(&format!("!{dir}/"))?;
    }
    Ok(overrides.build()?)
}

#[cfg(windows)]
fn should_skip_cloud(entry: &ignore::DirEntry) -> bool {
    use std::os::windows::fs::MetadataExt;
    const CLOUD_ATTRS: u32 = 0x00400000 | 0x00040000 | 0x00001000;
    entry
        .metadata()
        .map(|m| m.file_attributes() & CLOUD_ATTRS != 0)
        .unwrap_or(false)
}
