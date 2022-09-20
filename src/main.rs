use clap::{Parser, Subcommand};

pub mod check;
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
        /// Supports globing
        #[clap(default_value_t = String::from("*"))]
        file: String,
    },

    /// Runs a file
    run {
        /// File to run.
        #[clap(default_value_t = String::from("./main.c"))]
        file: String,
    },

    /// Runs tests contained within a particular test file or
    test {
        /// W.I.P. Wether to capture standard output or not.
        #[clap(short, long)]
        capture: bool,

        /// File to run tests from.
        #[clap(default_value_t = String::from("./test.c"))]
        file: String,

        /// Specific test to run.
        test: Option<String>,
    },

    /// Watches changes to source files and re run them
    watch {
        /// File to run.
        #[clap(default_value_t = String::from("./main.c"))]
        file: String,
    },
}

fn main() {
    let args: Arguments = Parser::parse();

    match args.command {
        Commands::check { file } => check::main(file),
        Commands::run { file } => {
            run::main(file);
        }
        Commands::test {
            capture,
            file,
            test,
        } => test::main(capture, file, test),
        Commands::watch { file } => watch::main(file),
    }
}
