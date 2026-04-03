use anyhow::Result;

use crate::util::path;

pub fn run() -> Result<()> {
    for dir in path::path_dirs() {
        println!("{dir}");
    }

    Ok(())
}
