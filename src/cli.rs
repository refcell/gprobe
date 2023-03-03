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
    /// Traverse a database
    Tree {
        /// The depth of the tree
        #[clap(short = 'l', long = "level", default_value = "3")]
        level: u64,

        /// The path of the database
        #[clap(short = 'p', long = "path")]
        path: Option<String>,
    },
    /// Create a new database
    Create {
        /// The path of the database
        #[clap(short = 'p', long = "path")]
        path: Option<String>,
    },
    /// Decompress a Tarball
    Decompress {
        /// The path of the tarball
        #[clap(short = 'p', long = "path")]
        path: Option<String>,
    },
}
