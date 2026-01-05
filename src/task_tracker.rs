use chrono::Utc;
use serde::{self, Deserialize, Serialize};
use serde_json::map::Map;
use std::{
    self,
    collections::HashMap,
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

static FILE_NAME: &str = "task-tracker.json";

#[derive(Debug)]
pub struct TaskTracker;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
struct Id(u128);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
struct Task {
    description: String,
    created_at: String,
    updated_at: String,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Status {
    ToDo,
    Done,
    InProgress,
}

pub enum Update {
    Status(Status),
    Description(String),
}

impl TaskTracker {
    pub fn builder() -> io::Result<()> {
        // Create a Serde Map and convert it into Pretty String
        let map = Map::new();
        let content =
            serde_json::to_string_pretty(&map).expect("Can't able to Serialize the String");

        // Check and Create the JSON file.
        let path = Path::new(FILE_NAME);
        match fs::exists(path) {
            Ok(result) => {
                if !result {
                    let file = fs::write(path, &content)
                        .expect("Couldn't able to add the content to the file");
                    Ok(file)
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }

    fn read() -> Result<HashMap<Id, Task>, serde_json::Error> {
        // Create Path and Read Content of the OutPut file
        let path = Path::new(FILE_NAME);
        let mut file = File::open(path).expect("File not found");
        let mut string = String::new();
        file.read_to_string(&mut string)
            .expect("Failed to Read the Content");
        match serde_json::from_str(&string) {
            Ok(result) => {
                let map: HashMap<Id, Task> = result;
                Ok(map)
            }
            Err(e) => Err(e),
        }
    }

    fn write(tasks: &HashMap<Id, Task>) {
        let tasks = serde_json::to_string_pretty(&tasks).expect("Failed to parse to string");
        fs::write(FILE_NAME, tasks).expect("Failed to write to file");
    }

    pub fn add(description: String) -> Result<u128, io::Error> {
        let id: Id;
        let mut content = Self::read().expect("Failed to Get the Tasks");
        let keys: Vec<_> = content.clone().into_keys().collect();
        id = if keys.len() == 0 {
            Id::new(keys.len() as u128)
        } else {
            Id::new(keys[keys.len() - 1].0 + 1 as u128)
        };
        let task = Task::new(description);
        content.insert(id.clone(), task);
        Self::write(&content);
        Ok(id.0)
    }

    /// This function is used to update the either the Status or the Description
    /// of the Task. The inputs which it will expect for are the Id and entry_type
    /// For entry Type use the enum Update.
    pub fn update(id: u128, entry_type: Update) {
        let mut content = Self::read().expect("Failed to Get the Tasks");
        let id = Id::new(id);
        if content.iter().count() > 0 {
            match entry_type {
                Update::Description(d) => {
                    if let Some(task) = content.get_mut(&id) {
                        task.update_description(d);
                        Self::write(&content);
                        println!("Successfully Update the Task Id:{} ", id.0)
                    } else {
                        println!("The Provided Id: {} is not valid", id.0);
                    }
                }
                Update::Status(s) => {
                    if let Some(task) = content.get_mut(&id) {
                        task.update_status(s);
                        Self::write(&content);
                    }
                }
            }
        } else {
            println!(
                "No Tasks to Update.
Try adding some tasks using: add Subcommand."
            );
        }
    }

    pub fn list_all() {
        let content = Self::read().expect("Failed to Get the Tasks");

        if content.iter().count() > 0 {
            for (key, value) in content.iter() {
                println!(
                    "Id: {} Description: {} Status: {:?}",
                    key.0, value.description, value.status
                );
            }
        } else {
            println!(
                "No Tasks to List.
Try adding some tasks using: add Subcommand."
            );
        }
    }

    pub fn list_by_status(status: Status) {
        let content = Self::read().expect("Failed to Get the Tasks");
        let mut counter = 0;
        if content.iter().count() > 0 {
            for (key, value) in content.iter() {
                if value.status == status {
                    counter += 1;
                    println!("Id: {}, Description: {}", key.0, value.description);
                }
            }
            if counter == 0 {
                println!("No Task related to the Status: {:?}", status);
            }
        } else {
            println!(
                "No Tasks to List.
Try adding some tasks using: add Subcommand."
            );
        }
    }

    pub fn delete_task(id: u128) {
        let id = Id::new(id);
        let mut content = Self::read().expect("Failed to Get the Tasks");
        if content.iter().count() > 0 {
            if content.contains_key(&id) {
                println!("Removing Task Id: {}", id.0);
                content.remove(&id);
                Self::write(&content);
            } else {
                eprintln!("Id: {} not found", id.0);
            }
        } else {
            println!(
                "No Tasks to delete.
Try adding some tasks using: add Subcommand."
            );
        }
    }
    #[allow(dead_code)]
    fn delete_all() {
        let mut content = Self::read().expect("Failed to Get the Tasks");
        content.drain();
        Self::write(&content);
    }
}

impl Id {
    fn new(id: u128) -> Self {
        Self(id)
    }
}

impl Task {
    fn new(description: String) -> Self {
        let date_time = Utc::now();
        Self {
            description,
            created_at: date_time.to_string(),
            updated_at: date_time.to_string(),
            status: Status::ToDo,
        }
    }

    /// Use this method to update the description of the task.
    /// Which will also update the updated_at field in the Task instance.
    fn update_description(&mut self, description: String) {
        let date_time = Utc::now();
        self.description = description;
        self.updated_at = date_time.to_string()
    }

    /// Use this method to update the status of the task.
    /// Which will also update the updated_at field in the Task instance.
    fn update_status(&mut self, status: Status) {
        let date_time = Utc::now();
        self.status = status;
        self.updated_at = date_time.to_string()
    }
}
