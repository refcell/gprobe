use std::thread;
use clap::CommandFactory;
use isatty::stdout_isatty;
use spinners::{Spinner, Spinners};

use crate::{cli::{GProbe, Subcommands}, telemetry};

impl GProbe {
    /// Runs the core logic for the [gprobe::cli::GProbe] CLI Application
    pub fn run(&mut self) {
        let mut app = GProbe::into_app();

        // Set up tracing
        let subscriber = match self.verbose {
            true => telemetry::get_subscriber("debug".into()),
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
            None => {
                if app.print_help().is_err() {
                    tracing::warn!("Failed to print help menu for gprobe command");
                }
            }
        }
    }

    /// Prints out a tree of the data source
    pub fn tree(&mut self, level: u64, path: Option<String>) {
        GProbe::spin("Probing...", || {
            tracing::debug!("Tracing a datastore tree with a depth of {}", level);
            thread::sleep(std::time::Duration::from_secs(2));

            let path = match path {
                Some(path) => path,
                None => {
                    tracing::warn!("No path provided for datastore tree");
                    return;
                }
            };
            crate::handlers::tree::walk(path, level);

            tracing::debug!("Done tracing datastore tree");
        });
    }

    /// Wraps the callback in a spinner if stdout is a TTY
    fn spin(prompt: impl std::fmt::Display, callback: impl FnOnce()) {
        // If stdout is a TTY, create a spinner
        let mut sp: Option<Spinner> = None;
        if stdout_isatty() {
            sp = Some(Spinner::new(Spinners::Dots, format!("{}", prompt)));
        }

        callback();

        // Stop spinner animation if it exists
        if let Some(mut sp) = sp {
            sp.stop();
            println!(" ");
        }
    }
}