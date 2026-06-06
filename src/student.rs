use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub birthdate: String, // 统一为 birthdate
    pub institute: String, // 修正拼写
    pub score: u8,
}

#[derive(Debug, Clone, Default)]
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
}
