use std::path::Path;

use rusty_leveldb::{DB, LdbIterator, Options};

/// Walks the tree in a depth-first manner, calling the given function on each node.
pub fn walk(
    path: impl AsRef<Path>,
    level: u64,
) {
    tracing::debug!("Walking db \"{}\" tree to a depth of {}", path.as_ref().to_string_lossy(), level);

    // Open the leveldb-rs database at the given path
    let open_options = Options { create_if_missing: false, ..Default::default() };
    let mut db = DB::open(path.as_ref(), open_options).unwrap();

    // Create a new db iterator
    let mut iter = match db.new_iter() {
        Ok(iter) => iter,
        Err(e) => {
            tracing::error!("Failed to create leveldb iterator: {}", e);
            return;
        }
    };

    // Check if we are valid
    tracing::debug!("Iterator is valid: {}", iter.valid());

    // Check the current value
    let curr_val = match iter.next() {
        Some(val) => val,
        None => {
            tracing::warn!("Iterator is not valid");
            return;
        }
    };

    // Print the current value
    tracing::debug!("Current value: {:?}", curr_val);
}