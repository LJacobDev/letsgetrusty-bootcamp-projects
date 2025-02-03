use std::io::Write;
use std::{fs::File, io::Read};

use anyhow::Result;

use crate::models::{DBState, Epic, Status, Story};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        //read the contents of self.file_path and deserialize it using serde
        let mut file = File::open(&self.file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        //the above three lines could have been all taken care of in one line as seen in the below comment:
        // let contents = fs::read_to_string(&self.file_path);

        let db_state: DBState = serde_json::from_str(&contents)?;
        Ok(db_state)

        // the following line also works as an alternative to creating a db_state binding and then using Ok() on it
        // Ok(serde_json::from_str(&contents)?)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        //serialize db_state to JSON and store it in self.file_path
        let serialized = serde_json::to_string(db_state)?;
        let mut file = File::create(&self.file_path)?;
        write!(file, "{}", serialized).unwrap();

        //according to the starter code in step 3, the above lines could have also been written in one like this:
        //fs::write(&self.file_path, &serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}


pub struct JiraDatabase {
    database: Box<dyn Database>
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        todo!()
    }

    pub fn read_db(&self) -> Result<DBState> {
        todo!()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        todo!()
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        todo!()
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        todo!()
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        todo!()
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        todo!()
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        todo!()
    }


}


pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap, hash::Hash};

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>
    }

    impl MockDB {
        pub fn new() -> Self {
            Self { last_written_state: RefCell::new(DBState { last_item_id: 0, epics: HashMap::new(), stories: HashMap::new()})}
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            // TODO: fix this error by deriving the appropriate traits for Story
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }
        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            // TODO: fix this error by deriving the appropriate traits for DBState
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::test_utils::MockDB;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use tempfile::tempfile;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_string(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            //NOTE:  the NamedTempFile's own doc string says that there are security implications that come with using this method, and they nearly just suggest not to use it
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;

            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;

            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let story = Story {
                name: "epic 1".to_string(),
                description: "epic 1".to_string(),
                status: Status::Open,
            };

            let epic = Epic {
                name: "epic 1".to_string(),
                description: "epic 1".to_string(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            eprint!("{}", db.file_path);

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            //I added this line because the test was failing on the assert_eq!(write_result.is_ok(), true); and I wanted to see what the error was saying.
            //The error from the current moment of debugging is saying that Access is denied (os error 5)
            //it was because I was using File::open in write_db instead of File::create
            eprint!("{:?}", write_result);

            assert_eq!(write_result.is_ok(), true);

            assert_eq!(read_result, state);
        }
    }
}
