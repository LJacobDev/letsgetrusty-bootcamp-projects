use std::collections::HashMap;

pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u16>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        todo!() //by default the status should be set to open, and the stories should be an empty vector
    }
}

pub struct Story {
    // TODO: add public fields
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        todo!() // by default the status should be set to open
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // add public fields
    pub last_item_id: u16,
    pub epics: HashMap<u16, Epic>,
    pub stories: HashMap<u16, Story>,
    // u16 used deliberately as a number with high enough max number but can save a few bits vs using u32
}