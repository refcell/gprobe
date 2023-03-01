use clap::Parser;

use gprobe::cli;

fn main() {
    cli::GProbe::parse().run();
}