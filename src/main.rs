use clap::{Parser, Subcommand};

pub mod check;
pub mod config;
pub mod run;
pub mod tasks;
pub mod test;
pub mod utils;
pub mod watch;

#[derive(Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    command: Commands,
}

// TODO: turn files into file lists

#[allow(non_camel_case_types)]
#[derive(Subcommand)]
pub enum Commands {
    /// Checks a source file for conformance with piscine limitations.
    check {
        /// File to check.
        files: Vec<String>,
    },

    /// Runs a file
    run {
        /// Files to run.
        files: Vec<String>,
    },

    /// Runs tests contained within a particular test file or
    test {
        /// W.I.P. Wether to capture standard output or not.
        #[clap(short, long)]
        capture: bool,

        /// Files to run tests from.
        files: Vec<String>,

        /// Specific test to run.
        test: Option<String>,
    },

    /// Watches changes to source files and re run them
    watch {
        /// Files to run.
        files: Vec<String>,
    },
}

fn main() {
    let args: Arguments = Parser::parse();

    match args.command {
        Commands::check { files } => check::main(files),
        Commands::run { files } => {
            run::main(files);
        }
        Commands::test {
            capture,
            files,
            test,
        } => test::main(capture, files, test),
        Commands::watch { files } => watch::main(files),
    }
}
