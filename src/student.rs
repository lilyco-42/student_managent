use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! set {
    (  $name:ident,$type:ty) => {
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = $name;
            self
        }
    };
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Student {
    name: String,
    age: u8,
    subjects: HashSet<String>,
    score: HashMap<String, u32>,
}

impl Student {
    pub fn new() -> Self {
        Self::default()
    }
    set!(name, String);
    set!(age, u8);
    // set!(score,HashMap<String,u32>);

    pub fn add_subjects(mut self, subject: HashSet<String>) -> Self {
        self.subjects.extend(subject);
        self
    }
}
