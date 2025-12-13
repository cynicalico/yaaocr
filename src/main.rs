use std::path::PathBuf;

use clap::{Parser, Subcommand};
use yaaocr::{aoc_proxy, runner};

/// Yet Another Advent of Code Runner
#[derive(Debug, Parser)]
#[clap(name = "yaaocr", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run solutions
    Run {
        /// Year to run
        #[arg(short, long, required = false)]
        year: Option<u32>,

        /// Day to run
        #[arg(short, long, required = false)]
        day: Option<u32>,

        /// Input file to use instead of the default
        #[arg(short, long, required = false, requires = "year", requires = "day")]
        input_path_override: Option<PathBuf>,

        /// Verify solutions
        #[arg(short, long, required = false)]
        verify: bool,

        /// Display totals
        #[arg(short, long, required = false)]
        totals: bool,
    },

    /// Download puzzles and inputs
    Download {
        /// Year to download
        year: u32,

        /// Day to download
        day: u32,

        /// What to download
        what: aoc_proxy::DownloadTarget,

        /// Force download
        #[arg(short, long, required = false)]
        force: bool,
    },
}

fn main() {
    let args = App::parse();

    if let Err(err) = match args.command {
        Command::Run {
            year,
            day,
            input_path_override,
            verify,
            totals,
        } => runner::run(year, day, input_path_override, verify, totals),
        Command::Download {
            year,
            day,
            what,
            force,
        } => aoc_proxy::download(year, day, what, force),
    } {
        eprintln!("Error: {err}");
    }
}
