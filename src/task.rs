use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub due_date: Option<NaiveDate>,
    pub completed: bool
}

