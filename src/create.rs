use std::path::Path;

use rusty_leveldb::{DB, LdbIterator, Options};
use spinners::Spinner;

/// Creates a new database at the provided path
pub fn create(
    path: impl AsRef<Path>,
    spinner: &mut Option<Spinner>
) {
    tracing::debug!("Creating db \"{}\"", path.as_ref().to_string_lossy());

    // Open the leveldb-rs database at the given path
    let open_options = Options { create_if_missing: true, ..Default::default() };
    let mut db = DB::open(path.as_ref(), open_options).unwrap();

    // Insert value and flush to disk
    db.put(b"key", b"value").unwrap();
    db.flush().unwrap();

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

    // Walk entire db
    while let Some((k, v)) = iter.next() {
        let key = String::from_utf8_lossy(k.as_slice());
        let value = String::from_utf8_lossy(v.as_slice());
        println!("Key: {}, Value: {}", key, value);
    }
}