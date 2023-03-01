use std::{path::Path, fs};
use eyre::Result;

/// Clean up the database directory
pub fn cleanup_db_dir(base: &str) -> Result<()> {
    delete_obsolete_files(base)?;
    let prefix = base.strip_suffix('/').unwrap_or(base);
    let dir = format!("{}/{}", prefix, "ancient");
    println!("Hoisting up a level: {}", dir);
    hoist_dir_up(&dir)?;
    Ok(())
}

/// Deletes obsolete files in a database directory.
pub fn delete_obsolete_files(base: &str) -> Result<()> {
    let prefix_path = base.strip_suffix('/').unwrap_or(base);
    let obsolete_files = vec![
        ".DS_Store",
        "CURRENT.bak",
    ];
    for file in obsolete_files {
        let file_path = format!("{}/{}", prefix_path, file);
        let path = Path::new(&file_path);
        if path.exists() {
            tracing::debug!("Deleting obsolete file {:?}", file_path);
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

/// Move a directory up a level in the filesystem
pub fn hoist_dir_up(dir: &str) -> Result<()> {
    let path = Path::new(dir);
    if path.exists() {
        tracing::debug!("Found directory {:?}", path);
        let parent = path.parent().unwrap().parent().unwrap();
        tracing::debug!("Hoisting up a level to \"{}\"...", parent.display());
        let new_path = parent.join(path.file_name().unwrap());
        tracing::debug!("Moving {:?} to {:?}", path, new_path);
        fs::rename(path, new_path)?;
    }
    Ok(())
}