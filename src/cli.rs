use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Adds new item to Todo
    Todo { item: String },
    /// Marks item as Done
    Done { item: Option<usize> },
    /// Adds new item to Notes
    Note { item: String },
    /// Print all todos or all notes
    Show {
        #[clap(subcommand)]
        subcommand: Option<TodoElements>,
    },
    /// Clears selected items
    Clear {
        #[clap(subcommand)]
        subcommand: Option<TodoElements>,
    }
}

#[derive(clap::Subcommand)]
pub enum TodoElements {
    Todo,
    Done,
    Note,
    All,
}

impl Default for TodoElements {
    fn default() -> Self {
        Self::All
    }
}
