use std::fs::File;
use chrono::{NaiveDate, Utc};

mod task;
mod task_map;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let task1 = task::Task {
        name: String::from("Finish Rust project"),
        due_date: Some(NaiveDate::from_ymd_opt(2026,2,9).unwrap()),
        completed: false,
    };

    let file = File::create("db/task.json")?;

    serde_json::to_writer(file, &task1)?;

    let json_file = File::open("db/task.json")?;

    let deserialized_task: task::Task = serde_json::from_reader(json_file)?;

    println!("Deserialized Task: {:?}", deserialized_task);

    Ok(())

}
