use std::{thread, fs::File};
use clap::CommandFactory;
use isatty::stdout_isatty;
use spinners::{Spinner, Spinners};
use flate2::read::GzDecoder;
use tar::Archive;

use crate::{cli::{GProbe, Subcommands}, telemetry};

impl GProbe {
    /// Runs the core logic for the [gprobe::cli::GProbe] CLI Application
    pub fn run(&mut self) {
        let mut app = GProbe::into_app();

        // Set up tracing
        let subscriber = match self.verbose {
            true => {
                println!("Initializing verbose subscriber...");
                telemetry::get_subscriber("debug".into())
            }
            false => telemetry::get_subscriber("info".into()),
        };
        telemetry::init_subscriber(subscriber);

        // Match on subcommand
        match &self.subcommand {
            Some(Subcommands::Tree { level, path }) => {
                let p = match self.source {
                    Some(_) => self.source.clone(),
                    None => path.clone(),
                };
                self.tree(*level, p);
            },
            Some(Subcommands::Create { path }) => {
                let p = match self.source {
                    Some(_) => self.source.clone(),
                    None => path.clone(),
                };
                self.create(p);
            },
            Some(Subcommands::Decompress { path }) => {
                let p = match self.source {
                    Some(_) => self.source.clone(),
                    None => path.clone(),
                };
                match p {
                    Some(p) => self.decompress(&p),
                    None => {
                        println!("No path provided for tarball");
                    }
                }
            },
            None => {
                if app.print_help().is_err() {
                    tracing::warn!("Failed to print help menu for gprobe command");
                }
            }
        }
    }

    /// Decompress a tarball
    pub fn decompress(&self, p: &str) {
        match File::open(p) {
            Ok(tar_gz) => {
                let tar = GzDecoder::new(tar_gz);
                let mut archive = Archive::new(tar);
                if let Err(e) = archive.unpack(".") {
                    tracing::error!("Failed to decompress tarball: {}", e);
                }
            },
            Err(e) => {
                tracing::error!("Failed to open tarball: {}", e);
            }
        }
    }

    /// Prints out a tree of the data source
    pub fn tree(&mut self, level: u64, path: Option<String>) {
        GProbe::spin("Probing...", |sp| {
            tracing::debug!("Tracing a datastore tree with a depth of {}", level);
            thread::sleep(std::time::Duration::from_secs(2));

            let path = match path {
                Some(path) => path,
                None => {
                    tracing::warn!("No path provided for datastore tree");
                    return;
                }
            };
            crate::handlers::walk(path, level, sp);

            tracing::debug!("Done tracing datastore tree");
        });
    }

    /// Creates a new database at the provided path
    pub fn create(&mut self, path: Option<String>) {
        GProbe::spin("Creating...", |sp| {
            thread::sleep(std::time::Duration::from_secs(2));

            let path = match path {
                Some(path) => path,
                None => {
                    tracing::warn!("No path provided for datastore tree");
                    return;
                }
            };
            crate::handlers::create(path, sp);

            tracing::debug!("Done tracing datastore tree");
        });
    }

    /// Wraps the callback in a spinner if stdout is a TTY
    fn spin(prompt: impl std::fmt::Display, callback: impl FnOnce(&mut Option<Spinner>)) {
        // If stdout is a TTY, create a spinner
        let mut sp: Option<Spinner> = None;
        if stdout_isatty() {
            sp = Some(Spinner::new(Spinners::Dots, format!("{}", prompt)));
        }

        callback(&mut sp);

        // Stop spinner animation if it exists
        if let Some(mut sp) = sp {
            sp.stop();
            println!(" ");
        }
    }
}