use anyhow::Result;

use crate::util::{path, ui};

pub fn run(query: &str) -> Result<()> {
    let results: Vec<String> = path::search_executables(query)
        .into_iter()
        .map(|p| p.display().to_string())
        .collect();

    if results.is_empty() {
        ui::warn(&format!(
            "No executables matching '{}' found on PATH.",
            query
        ));
    } else {
        ui::info(&format!("Executables matching '{}':", query));
        for line in ui::format_tree(&results) {
            eprintln!("{line}");
        }
    }

    Ok(())
}
