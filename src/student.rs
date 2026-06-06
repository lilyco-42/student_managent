use serde::{Deserialize, Serialize};

use crate::student;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub birthdate: String,
    pub institute: String,
    pub score: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StudentList {
    pub students: Vec<Student>,
}

impl StudentList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn list(&self) -> &Vec<Student> {
        &self.students
    }
    pub fn get_by_id(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }
    pub fn delete_by_id(&mut self, id: u32) -> bool {
        let len_before = self.students.len();
        self.students.retain(|s| s.id != id);
        len_before != self.students.len()
    }

    pub fn update(&mut self, updated: Student) -> bool {
        if let Some(existing) = self.students.iter_mut().find(|s| s.id == updated.id) {
            *existing = updated;
            true
        } else {
            false
        }
    }
}
