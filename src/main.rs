use chrono::{NaiveDate};
use clap::{Parser, Subcommand};
use crate::tasks::task_map::TaskMap;
use crate::tasks::task::Task;

mod tasks;

const FILE_PATH: &str = "db/task.json";

#[derive(Parser, Debug)]
#[command(
    name = "Task Manager",
    version = "1.0",
    author = "Sean Ouellette",
    about = "A simple task management CLI application")
]
struct Cli {

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {

    // The "add" command allows the user to add a new task to the task list.
    #[command(name = "add", about = "Add a new task to the task list")]
    Add {
        #[arg(short, long, help = "The name of the task to be added")]
        name: String,

        #[arg(short, long, help = "The subject or category of the task (optional)")]
        subject: Option<String>,

        #[arg(short, long, help = "The due date for the task in YYYY-MM-DD format (optional)")]
        due_date: Option<NaiveDate>,
    },

    // The "list" command allows the user to view all tasks currently in the task list.
    #[command(name = "list", about = "List all tasks in the task list")]
    List,

    // The "complete" command allows the user to mark a specific task as completed by providing its name.
    #[command(name = "complete", about = "Mark a task as completed")]
    Complete {
        #[arg(short, long, help = "The name of the task to be marked as completed")]
        name: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();

    match cli.command {
        Command::Add { name, subject, due_date } => {
            // Create a new Task object with the provided name, subject, and due date.
            let task = Task::new(name, subject, due_date);
            
            // Load the existing tasks from the JSON file into a TaskMap. 
            //  If the file doesn't exist or is empty, start with an empty TaskMap.
            let mut task_map = TaskMap::new();
            task_map.load(FILE_PATH)?;
            
            // Add the new task to the TaskMap and save the updated task list back to the JSON file.
            task_map.add_task(task);
            task_map.save(FILE_PATH)?;
        },
        
        Command::List => {
            // Load the existing tasks from the JSON file into a TaskMap. 
            //  If the file doesn't exist or is empty, start with an empty TaskMap.
            let mut task_map = TaskMap::new();
            task_map.load(FILE_PATH)?;
            
            // Print the current tasks in the TaskMap. 
            //  If there are no tasks, print a message indicating that the task list is empty.
            println!("Current tasks:");
            println!("{}", task_map.to_string());
        },
        
        Command::Complete { name } => {
            // Load the existing tasks from the JSON file into a TaskMap. 
            //  If the file doesn't exist or is empty, start with an empty TaskMap.
            let mut task_map = TaskMap::new();
            task_map.load(FILE_PATH)?;
            
            // Attempt to find the task by name. If found, mark it as completed and save the updated task list. 
            //  If not found, print an error message.
            if let Some(task) = task_map.get_task(&name) {
                task.mark_completed();
                task_map.save(FILE_PATH)?;
                println!("Task '{}' marked as completed.", name);
            } else {
                println!("Task '{}' not found.", name);
            }
        },
    }

    Ok(())
}
