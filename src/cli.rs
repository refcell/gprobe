
use clap::{Parser, Subcommand};

/// GProbe Cli Args
#[derive(Parser, Debug, Clone)]
#[clap(name = "grpobe", version, about, long_about = None)]
pub struct GProbe {
    /// The data source to probe.
    pub source: Option<String>,

    /// Prints out to the terminal.
    #[clap(short = 'p', long = "print")]
    pub print: bool,

    /// Verbose output.
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// GProbe Subcommands
    #[clap(subcommand)]
    pub subcommand: Option<Subcommands>,
}

/// GProbe Subcommands
#[derive(Subcommand, Clone, Debug)]
pub enum Subcommands {
    /// tree subcommand
    Tree {
        /// The depth of the tree
        #[clap(short = 'l', long = "level", default_value = "3")]
        level: u64,
    },
}