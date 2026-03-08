use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use crate::tasks::task::Task;

/// A structure to hold a map of tasks, allowing for easy retrieval and management.
pub struct TaskMap {
    pub map: HashMap<String, Task>
}

impl TaskMap {
    /// Creates a new, empty TaskMap.
    pub fn new() -> Self {
        TaskMap {
            map: HashMap::new()
        }
    }

    /// Adds a task to the TaskMap, using the task's name as the key.
    pub fn add_task(&mut self, task: Task) {
        // Check if a task with the same name already exists in the map.
        //  If it does, print a warning message and overwrite the existing task.
        //  Otherwise, add the new task to the map.
        if self.map.contains_key(&task.get_name()) {
            println!("Task with name '{}' already exists. Overwriting existing task.", task.get_name());
        } else {
            self.map.insert(task.get_name(), task);
        }
    }

    /// Retrieves a reference to a task by its name. Returns None if the task does not exist in the map.
    pub fn get_task(&mut self, name: &str) -> Option<&mut Task> {
        self.map.get_mut(name)
    }

    /// Loads tasks from a JSON file and adds them to the TaskMap. The JSON file should contain an array of tasks.
    pub fn load(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

        // Check if the file exists. If it does not, create it and return early to avoid deserialization errors.
        if !Path::new(filename).exists() {
            // If the file does not exist, create it and return early
            println!("File '{}' does not exist. Creating a new file.", filename);
            File::create(filename)?;
            return Ok(());
        }

        // Try to open the file. I fit does not error
        let file = File::open(filename)?;

        // Check if the file is empty. If it is, return early to avoid deserialization errors.
        if file.metadata()?.len() == 0 {
            return Ok(());
        }

        // Deserialize the JSON data into a vector of Task objects and add them to the TaskMap
        let tasks: Vec<Task> = serde_json::from_reader(file)?;
        for task in tasks {
            self.add_task(task);
        }
        Ok(())
    }

    /// Saves the current tasks in the TaskMap to a JSON file. The tasks are saved as an array of task objects.
    pub fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

        let tasks: Vec<&Task> = self.map.values().collect();

        let json = serde_json::to_string_pretty(&tasks)?;
        fs::write(filename, json)?;
        Ok(())
    }

    /// Converts the TaskMap to a formatted string for display purposes.
    ///     The string includes a header and a row for each task, showing the name, subject, due date, and completion status.
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let name_col_width = self.map.keys().map(|k| k.len()).max().unwrap_or(18).max(18) + 2;
        result.push_str(
            &format!(
                "|{:^name_col_width$}|{:^20}|{:^20}|{:^20}|\n",
                "Name",
                "Subject",
                "Due Date",
                "Completed",
                name_col_width = name_col_width
            ).as_str()
        );
        result.push_str(
            format!(
                "|{:^name_col_width$}|{:^20}|{:^20}|{:^20}|\n",
                "-".repeat(name_col_width),
                "-".repeat(20),
                "-".repeat(20),
                "-".repeat(20),
                name_col_width = name_col_width
            ).as_str()
        );
        for (_, task) in &self.map {
            result.push_str(
                &format!(
                    "|{:^name_col_width$}|{:^20}|{:^20}|{:^20}|\n",
                    task.get_name(),
                    task.get_subject().unwrap_or_else(|| "None".to_string()),
                    task.get_due_date().map_or_else(|| "None".to_string(), |d| d.to_string()),
                    if task.is_completed() { "Yes" } else { "No" },
                    name_col_width = name_col_width
                ).as_str()
            );
        }
        result
    }
}