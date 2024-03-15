mod commands;

use std::io;

use clap::{ Parser, Subcommand, command };

#[derive(Parser, Debug)]
struct Cli
{
    ///Main command to execute
    #[command(subcommand)]
    command: Option<Command>
}


#[derive(Subcommand, Debug)]
enum Command
{
    /// Adds a todo into the list
    #[command()]
    Add
    {
        /// The todo message
        #[arg(required=true)]
        todo: Vec<String>
    },
    /// Mark a specific todo as complete
    #[command()]
    MarkDone
    {
        /// The todo id to mark as complete
        #[command()]
        id: usize
    },
    /// List all the todos 
    #[command()]
    List,
    
    /// Delete a entry by id
    #[command()]
    Delete
    {
        /// Message index to delete
        #[arg()]
        id: usize,
        #[arg(short='l', default_value_t=false, required=false)]
        list: bool
    }
}

fn main() {
    let args = Cli::parse();
    match args.command
    {
        Some(Command::Add { todo }) => 
        {
            match commands::add(todo)
            {
                Ok(_) => { },
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied =>
                {
                    eprintln!("Permission denied to file!")
                },
                Err(e) => 
                {
                    eprintln!("Couldnt add todo to file: {}", e.kind())
                }
            }
        },
        Some(Command::MarkDone { id }) => 
        {
            commands::mark_done(id)
        },
        Some(Command::List) => 
        {
            commands::list()
        },
        Some(Command::Delete { id, list }) => 
        {
            if commands::delete(id).is_err()
            {
                eprintln!("Something bad occured when trying to delete");
            }
            else if list 
            {
                commands::list()
            }
        }
        None =>
        {
            commands::list()
        }
    }
}
