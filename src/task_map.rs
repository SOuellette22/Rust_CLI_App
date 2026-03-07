use std::collections::HashMap;
use std::fs;
use std::fs::File;
use crate::task::Task;

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
        self.map.insert(task.get_name().clone().parse().unwrap(), task);
    }

    /// Retrieves a reference to a task by its name. Returns None if the task does not exist in the map.
    pub fn get_task(&mut self, name: &str) -> Option<&mut Task> {
        self.map.get_mut(name)
    }

    /// Loads tasks from a JSON file and adds them to the TaskMap. The JSON file should contain an array of tasks.
    pub fn load(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

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

    pub fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

        let tasks: Vec<&Task> = self.map.values().collect();

        let json = serde_json::to_string_pretty(&tasks)?;
        fs::write(filename, json)?;
        Ok(())
    }
}