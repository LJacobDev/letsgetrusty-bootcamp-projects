use std::{fs::File, io::Read};
use std::io::Write;

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
        let db_state: DBState = serde_json::from_str(&contents)?;
        Ok(db_state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        //serialize db_state to JSON and store it in self.file_path
        let serialized = serde_json::to_string(db_state)?;
        let mut file = File::create(&self.file_path)?;
        write!(file, "{}", serialized).unwrap();
        Ok(())

    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
                    .to_string()
            };

            let story = Story { name: "epic 1".to_string(), description: "epic 1".to_string(), status: Status::Open };

            let epic = Epic {name: "epic 1".to_string(), description: "epic 1".to_string(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };
            
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
