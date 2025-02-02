use std::io::{self, Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    todo: Vec<String>,
    done: Vec<String>,
    note: Vec<String>,
}

impl Todo {
    /// Displays the current TODO lists.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo show todo
    /// -- TODO --
    ///    1. Wash dog
    ///    2. Wash car
    /// ```
    pub fn handle_show_todo(&self) -> anyhow::Result<()> {
        if self.todo.is_empty() {
            return Ok(());
        }
        println!("-- TODO --");
        for (i, todo) in self.todo.iter().enumerate() {
            println!("   {i}. {todo}");
        }
        Ok(())
    }

    /// Displays the current NOTE lists.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo show note
    /// -- NOTE --
    ///    1. Wash dog
    ///    2. Wash carnotes
    /// ```
    pub fn handle_show_note(&self) -> anyhow::Result<()> {
        if self.note.is_empty() {
            return Ok(());
        }
        println!("-- NOTE --");
        for (i, note) in self.note.iter().enumerate() {
            println!("   {i}. {note}");
        }
        Ok(())
    }

    /// Displays the current DONE lists.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo show done
    /// -- DONE --
    ///    1. Wash dog
    ///    2. Wash car
    /// ```
    pub fn handle_show_done(&self) -> anyhow::Result<()> {
        if self.done.is_empty() {
            return Ok(());
        }
        println!("-- DONE --");
        for (i, todo) in self.done.iter().enumerate() {
            println!("   {i}. {todo}");
        }
        Ok(())
    }

    /// Displays the current TODO, DONE, NOTE lists.
    pub fn handle_show_all(&self) -> anyhow::Result<()> {
        self.handle_show_todo()?;
        self.handle_show_done()?;
        self.handle_show_note()?;
        Ok(())
    }

    /// Moves an item from the TODO list to the DONE list.
    ///
    /// # Arguments
    /// * `done_item` - The item to mark as done. This can be either the task description or its index.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo done "Wash dog"  # Marks "Wash dog" as done
    /// ```
    pub fn handle_done(&mut self, index: Option<usize>) -> anyhow::Result<()> {
        // if done_item.trim().is_empty() {
        //     self.handle_show_todo()?;
        //     println!("Choose item to mark as done. (Default 1) :");
        //     io::stdout().flush()?;
        //     let mut buffer = String::new();
        //     io::stdin().read_line(&mut buffer)?;
        //     let input: i32 = buffer.trim().parse().unwrap_or(-1);

        //     println!("DEBUG: {}", input);
        // }

        // if let Some(idx) = self.todo.iter().position(|item| done_item == *item) {
        //     let item = self.todo.remove(idx);
        //     println!("Removing `{}` from todos!", item);
        //     self.done.push(item);
        // }
        // Ok(())

        let item = if index.is_none() {
            self.handle_show_todo()?;
            println!("Choose item to mark as done. (Default 0) :");
            io::stdout().flush()?;
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let index: usize = buffer.trim().parse().unwrap_or(0);
            self.todo.remove(index)
        } else {
            self.todo.remove(index.unwrap())
        };

        println!("Removing `{}` from todos!", item);

        self.done.push(item);

        Ok(())
    }

    /// Adds a new item to the TODO list if it doesn't already exist.
    ///
    /// # Arguments
    /// * `todo_item` - The task to be added to the TODO list.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo todo "Wash dog"  # Adds "Wash dog" to the TODO list
    /// ```
    pub fn handle_todo(&mut self, todo_item: String) -> anyhow::Result<()> {
        println!("Adding `{}` to todos!", todo_item);
        if !self.todo.contains(&todo_item) && !todo_item.trim().is_empty() {
            self.todo.push(todo_item.trim().to_string());
        }
        Ok(())
    }

    /// Adds a new item to the NOTE list if it doesn't already exist.
    ///
    /// # Arguments
    /// * `note_item` - The task to be added to the NOTE list.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo note "Wash dog"  # Adds "Wash dog" to the NOTE list
    /// ```
    pub fn handle_note(&mut self, note_item: String) -> anyhow::Result<()> {
        println!("Adding `{}` to notes!", note_item);
        if !self.note.contains(&note_item) && !note_item.trim().is_empty() {
            self.note.push(note_item.trim().to_string());
        }
        Ok(())
    }

    /// Clears all items from the DONE list after user confirmation.
    ///
    /// Prompts the user to confirm the action. If the user inputs "n" or "N", the action is aborted.
    /// Any other input (or no input) proceeds with clearing.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo clear_done
    /// Clearing done. Is this what you want to do? (Default Y/n): Y
    /// ✅ Done list cleared.
    /// ```
    pub fn handle_clear_done(&mut self) -> anyhow::Result<()> {
        print!("Clearing done. Is this what you want to do? (Default Y/n): ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim();

        if input.eq_ignore_ascii_case("n") {
            println!("Aborted clearing done items.");
            return Ok(());
        }

        self.done.clear();
        println!("✅ Done list cleared.");

        Ok(())
    }

    /// Clears all items from the TODO list after user confirmation.
    ///
    /// Prompts the user to confirm the action. If the user inputs "n" or "N", the action is aborted.
    /// Any other input (or no input) proceeds with clearing.
    ///
    /// # Example
    /// ```shell
    /// $ rstodo clear_todo
    /// Clearing todo. Is this what you want to do? (Default Y/n): n
    /// Aborted clearing todo items.
    /// ```
    pub fn handle_clear_todo(&mut self) -> anyhow::Result<()> {
        print!("Clearing todo. Is this what you want to do? (Default Y/n): ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim();

        if input.eq_ignore_ascii_case("n") {
            println!("Aborted clearing todo items.");
            return Ok(());
        }

        self.todo.clear();
        println!("✅ Todo list cleared.");

        Ok(())
    }

    pub fn handle_clear_note(&mut self) -> anyhow::Result<()> {
        print!("Clearing notes. Is this what you want to do? (Default Y/n): ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim();

        if input.eq_ignore_ascii_case("n") {
            println!("Aborted clearing note items.");
            return Ok(());
        }

        self.note.clear();
        println!("✅ Note list cleared.");

        Ok(())
    }

    pub fn handle_clear_all(&mut self) -> anyhow::Result<()> {
        self.handle_clear_todo()?;
        self.handle_clear_done()?;
        self.handle_clear_note()?;
        Ok(())
    }
}
