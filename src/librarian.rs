// Keeps track of what files exist and if not where they are

use std::{collections::HashMap, sync::{Arc, Mutex}, ops::DerefMut, hash::Hash};
use serde::{Serialize, Deserialize};

// Like s3, this refers to a file
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Object {
    name: String,
    mime: String,
    in_fs: bool, // Does it exist on this server?
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub files: HashMap<String, Object>
}

impl Library {
    pub fn new() -> Library {
        return match std::fs::read_to_string("lib.json") {
            Ok(f) => {
                serde_json::from_str::<Library>(&f).unwrap()
            },
            Err(_) => {
                Library {
                    files: HashMap::new(),
                }
            }
        }
    }

    // TODO: implement MIME
    pub fn add_object(&mut self, name: String, in_fs: bool) {
        self.files.insert(name.clone(), Object {
            name: name.clone(),
            in_fs,
            mime: String::from("unknown"),
        });

        std::fs::write("lib.json",
            serde_json::to_string(self).unwrap()  
        );

    }
}