use clap::{Parser, Subcommand};

pub(crate) mod commune;
pub(crate) mod completion;
pub(crate) mod department;

/// Application for finding farmers in France
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    name = "annuma",
    bin_name = "annuma",
    rename_all = "kebab-case"
)]
pub struct Annuma {
    #[command(subcommand)]
    pub cmd: Commands,
}

/// Doc comment
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new Map
    Map {
        #[command(subcommand)]
        department: department::Department,
    },
    /// Create a new Map of Poste Source
    PosteSource,
    /// Installs completion script
    Completion,
}
