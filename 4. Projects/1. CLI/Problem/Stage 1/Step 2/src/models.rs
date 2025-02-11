use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u64>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DBState {
    pub last_item_id: u64,
    pub epics: HashMap<u64, Epic>,
    pub stories: HashMap<u64, Story>,
}
