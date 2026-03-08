use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

/// Represents a task with a name, optional subject, optional due date, and completion status.
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    name: String,
    subject: Option<String>,
    due_date: Option<NaiveDate>,
    completed: bool
}

/// Implementation of the Task struct, including constructors, getters, and helper functions.
impl Task {

    /// Creates a new Task with the given name, optional subject, and optional due date. The task is initially marked as incomplete.
    pub fn new(name: String, subject: Option<String>, due_date: Option<NaiveDate>) -> Self {
        Task {
            name,
            subject,
            due_date,
            completed: false
        }
    }

    // ---- Getters ---- //

    /// Returns the name of the task.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Returns the subject of the task, if it exists.
    pub fn get_subject(&self) -> Option<String> {
        self.subject.clone()
    }

    /// Returns the due date of the task, if it exists.
    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }

    /// Returns whether the task is completed.
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    // ---- Helper Functions ---- //

    /// Marks the task as completed.
    pub fn mark_completed(&mut self) {
        self.completed = !self.completed;
    }


}