use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Initialise a new repository
    Init,

    CatFile {
        /// Pretty print the object
        #[arg(short)]
        pretty_print: bool,

        /// The object to cat
        object: String,
    }
}
