use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    name: String,
    subject: Option<String>,
    due_date: Option<NaiveDate>,
    completed: bool
}

impl Task {

    pub fn new(name: String, subject: Option<String>, due_date: Option<NaiveDate>) -> Self {
        Task {
            name,
            subject,
            due_date,
            completed: false
        }
    }

    // ---- Getters ---- //

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_subject(&self) -> Option<String> {
        self.subject.clone()
    }

    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    // ---- Helper Functions ---- //

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }

    pub fn mark_incomplete(&mut self) {
        self.completed = false;
    }


}