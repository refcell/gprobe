use std::path::Path;

use rusty_leveldb::{LdbIterator, Options, DB};
use spinners::Spinner;

use crate::utils;

/// Walks the tree in a depth-first manner, calling the given function on each node.
pub fn walk(path: impl AsRef<Path>, level: u64, spinner: &mut Option<Spinner>) {
    let db_name = path.as_ref().to_string_lossy();
    tracing::debug!("Walking db \"{}\" tree to a depth of {}", db_name, level);

    // Cleanup the database directory
    if let Err(e) = utils::cleanup_db_dir(&db_name) {
        tracing::error!("Failed to cleanup database directory: {}", e);
        return;
    }

    // Open the leveldb-rs database at the given path
    let open_options = Options {
        create_if_missing: false,
        ..Default::default()
    };
    let mut db = DB::open(path.as_ref(), open_options).unwrap();

    // Create a new db iterator
    let mut iter = match db.new_iter() {
        Ok(iter) => iter,
        Err(e) => {
            tracing::error!("Failed to create leveldb iterator: {}", e);
            return;
        }
    };

    // Stop the spinner so we can print
    if let Some(sp) = spinner {
        sp.stop();
        println!();
    }
    *spinner = None;

    // Warn if db is empty
    if iter.next().is_none() {
        tracing::warn!("Error: Database is empty");
    }
    iter.reset();

    // Walk the database
    while let Some((k, v)) = iter.next() {
        let key = format!("0x{}", hex::encode(&k));
        let value = format!("0x{}", hex::encode(&v));
        println!("Key: {}\nValue: {}\n", key, value);
    }
    println!("Finished walking db \"{}\"", db_name);
}
