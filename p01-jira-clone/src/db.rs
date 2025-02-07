use std::io::Write;
use std::{fs::File, io::Read};

use anyhow::{anyhow, Context, Result};

use crate::db;
use crate::models::{DBState, DbIndex, Epic, Status, Story};

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
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        //self contains a JSONFileDatabase which itself has read_db() which reads its file path and returns the Result<DBState> that needs to be returned here
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<DbIndex> {
        // just initial thinking about it, it looks like you're handed an Epic and so you'll want to take a DBState and increment the last_item id and then use that as a key in the epics HashMap to insert it there
        // by looking at the unit test for this, it is clear that it intends to take a DBState type value and increment its last_item_id counter as well as insert the given epic into its epics HashMap,
        // and then to return the last_item_id in a Result<> also.  The unit test then checks that the returned value is the same as a db_state's last_item_id and makes sure its .epics HashMap has the expected epic at the key of its id number
        // to get that DBState I would use read_db and to save a DBstate change I use write_db

        //match statement used here because I wanted to deliberately use the anyhow! macro as advised in 'note 1' of the instructions
        match self.database.read_db() {
            Ok(mut db_state) => {
                db_state.last_item_id += 1;
                db_state.epics.insert(db_state.last_item_id, epic);
                self.database.write_db(&db_state).context("Error writing to the database in create_epic")?;
                Ok(db_state.last_item_id)
            },
            Err(e) => Err(anyhow!("Error reading database: {}", e))
        }

    }

    pub fn create_story(&self, story: Story, epic_id: DbIndex) -> Result<DbIndex> {
        // todo!()
        //read_db to get dbstate
        //increment the last_item_id
        //insert a new story with that id into stories
        //get the epic at epic_id and append last_item_id to its stories vector to map this story's id to that epic
        //write_db to write new dbstate to db
        //return the last_item_id in an OK
        //use either try operators or anyhow! macros to handle errors

        let mut db_state = self.database.read_db()?;


        //using different syntax than usual just to try things out


        //before doing anything at all, make sure that the intended epic_id exists in db_state.epics or else return an error without changing anything
        let mut epic = db_state.epics.get_mut(&epic_id);

        if let None = epic {
            return Err(anyhow!("Epic_id not found, story creation aborted"));
        }

        //shadowing epic with its own unwrap, becuase it is certain to not be a None by this point
        let epic = epic.unwrap();
        db_state.last_item_id += 1;
        db_state.stories.insert(db_state.last_item_id, story);

        epic.stories.push(db_state.last_item_id);

        self.database.write_db(&db_state)?;

        Ok(db_state.last_item_id)
    }

    pub fn delete_epic(&self, epic_id: DbIndex) -> Result<()> {
        // todo!()
        //read_db to get a dbstate to mutate
        //delete epic_id key from dbstate.epics hashmap (error handling if key doesn't exist)
        //write_db
        //return Ok(())
        //there will still be the stories that this epic was a part of, now without an associated epic to link them to
        //you might want to delete all its stories by calling delete_story on each id in its vector before removing it
        
        // * the unit test for this is expecting all the stories in the epic to no longer exist as well

        let mut db_state = self.database.read_db()?;

        eprintln!("{:?}", db_state);
        

        //NOTE:  there is an issue with needing to remove the story from the epic's vector of stories, but I'm iterating on that same vector
        //       so I might clone the list of story indexes or values so I can iterate over one and modify the other independently

        //       What I'll do:
        //       clone the stories vector to iterate over,
        //       clear the original stories vector before calling delete story on the iterated list of story ids
        //       now when delete_story is tested, epic will no longer contain it because it was cleared
        //       but the iterations across all contained stories can still happen

        //the epic needs to persist until the stories have been deleted, then remove the epic

        let epic = db_state.epics.get_mut(&epic_id);

        eprintln!(" epic is a: {epic:?}");

        if epic == None {
            return Err(anyhow!("No epic found at this epic id"));
        }

        let epic = epic.unwrap();

        let story_ids = epic.stories.clone();
        epic.stories.clear();

        db_state.epics.remove(&epic_id);
        
        for story in story_ids {
            //the delete_story test requires that the epic still exist and that it no longer contains the story_id in order for it to pass
            //it involves removing the story from epic.stories while making sure that doing so won't compromise the iteration of the for loop itself
            // epic.stories.retain(|id| *id != story);
            self.delete_story(epic_id, story)?;
        }


        eprintln!("{:?}", db_state);

        self.database.write_db(&db_state)?;
        
        Ok(())
    }

    pub fn delete_story(&self, epic_id: DbIndex, story_id: DbIndex) -> Result<()> {
        // todo!()
        //read_db
        //delete story_id from stories hashmap
        //remove story_id from stories vec in epic hashmap at key epic_id
        //write_db

        // thought process:
        // you would think that this method should remove the story from the stories hashmap, but also remove the story_id from the epic's stories vector
        // but I need to think about that more because delete_story is also being called from delete_epic so the epic might already be gone by the time this is being called
        // and I might need to rewrite that to only remove the epic once it has first deleted all its stories

        let mut db_state = self.database.read_db()?;

        if let None = db_state.epics.get(&epic_id) {
            return Err(anyhow!("No epic found with this epic_id"));
        }

        //this code block is expecting db_state to mutate so that the story at story_id is no longer present,
        //but the cargo test run is showing that the story is still contained inside of it even though it's saying "story_id removed successfully"
        eprintln!("{db_state:?}");
        match db_state.stories.remove(&story_id) {
            Some(_) => {

                eprintln!("{db_state:?}");
                println!("Story_ID {story_id} removed successfully");
                eprintln!("immediate db_state is now: {db_state:?}");
                self.database.write_db(&db_state)?;
                Ok(())
            },
            None => {
                Err(anyhow!("No story found at this story id"))
            }
        }

    }

    pub fn update_epic_status(&self, epic_id: DbIndex, status: Status) -> Result<()> {
        todo!()
        //read_db
        //get mutable reference to epics hashmap at key epic_id and assign status to its status field
        //write_db
        //return Ok(())
    }

    pub fn update_story_status(&self, story_id: DbIndex, status: Status) -> Result<()> {
        todo!()
        //read_db
        //get mutable reference to stories hashmap at story_id key
        //assign status to the story.status field
        //write_db
        //return Ok(())

    }
}

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap, hash::Hash};

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    impl MockDB {
        pub fn new() -> Self {
            Self {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            // fix this error by deriving the appropriate traits for Story
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }
        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            // fix this error by deriving the appropriate traits for DBState
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::test_utils::MockDB;
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

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        // fix this error by deriving the appropriate traits for Epic
        let result = db.create_epic(epic.clone());

        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        // fix this error by deriving the appropriate traits for Story
        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(result.is_ok(), true);

        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(
            db_state.epics.get(&epic_id).unwrap().stories.contains(&id),
            true
        );
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let non_existent_epic_id = 999;

        let result = db.delete_story(non_existent_epic_id, story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let non_existent_story_id = 999;

        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(
            db_state
                .epics
                .get(&epic_id)
                .unwrap()
                .stories
                .contains(&story_id),
            false
        );
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = db.update_story_status(story_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(
            db_state.stories.get(&story_id).unwrap().status,
            Status::Closed
        );
    }
}
