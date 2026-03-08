use chrono::{NaiveDate};
use crate::tasks::task_map::TaskMap;
use crate::tasks::task::Task;

mod tasks;

const FILE_PATH: &str = "db/task.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let task1 = Task::new(
        "Buy groceries".to_string(),
        Some("Personal".to_string()),
        Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap())
    );

    let task2 = Task::new(
        "Testing".to_string(),
        None,
        Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()),
    );

    let mut task_map = TaskMap::new();
    task_map.load(FILE_PATH)?;
    task_map.add_task(task1);
    task_map.add_task(task2);

    let task = task_map.get_task("Personal_Buy groceries");
    task.unwrap().mark_completed();

    println!("Current tasks:");
    for (name, task) in &task_map.map {
        println!("Task: {}, Due: {:?}, Completed: {}", name, task.get_due_date(), task.is_completed());
    }

    task_map.save(FILE_PATH)?;

    Ok(())

}
