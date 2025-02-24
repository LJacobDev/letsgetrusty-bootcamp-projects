use std::collections::HashMap;

use serde::{Deserialize, Serialize};

//derive the appropriate traits
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub type DbIndex = u16;

//derive the appropriate traits
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<DbIndex>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        //by default the status should be set to open, and the stories should be an empty vector
        Epic {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

//derive the appropriate traits
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Story {
            name,
            description,
            status: Status::Open,
        }
    }
}

//derive the appropriate traits
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // add public fields
    pub last_item_id: DbIndex,
    pub epics: HashMap<DbIndex, Epic>,
    pub stories: HashMap<DbIndex, Story>,
    // u16 used deliberately as a number with high enough max number but can save a few bits vs using u32
}