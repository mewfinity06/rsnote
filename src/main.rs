use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

pub mod cli;
pub mod todo;

use cli::Cli;
use todo::Todo;

fn read_todo_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Todo> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Todo`.
    let u = serde_json::from_reader(reader)?;

    // Return the `Todo` struct.
    Ok(u)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;
    let mut todo = read_todo_from_file("todo.json")?;

    match cli.command {
        cli::Command::Todo { item } => todo.handle_todo(item)?,
        cli::Command::Done { item } => todo.handle_done(item)?,
        cli::Command::Note { item } => todo.handle_note(item)?,
        cli::Command::Clear { subcommand } => {
            if let Some(subcommand) = subcommand {
                match subcommand {
                    cli::TodoElements::Todo => todo.handle_clear_todo()?,
                    cli::TodoElements::Done => todo.handle_clear_done()?,
                    cli::TodoElements::Note => todo.handle_clear_note()?,
                    cli::TodoElements::All => todo.handle_clear_all()?,
                }
            } else {
                todo.handle_clear_all()?;
            }
        }
        cli::Command::Show { subcommand } => {
            if let Some(subcommand) = subcommand {
                match subcommand {
                    cli::TodoElements::Todo => todo.handle_show_todo()?,
                    cli::TodoElements::Done => todo.handle_show_done()?,
                    cli::TodoElements::Note => todo.handle_show_note()?,
                    cli::TodoElements::All => todo.handle_show_all()?,
                }
            } else {
                todo.handle_show_all()?;
            }
            // There is no point in rewriting the .json object if there is no update
            return Ok(());
        }
    }

    let json = serde_json::to_string_pretty(&todo).unwrap();

    let mut file = File::create("todo.json")?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
