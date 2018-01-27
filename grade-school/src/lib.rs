use std::collections::HashMap;

#[allow(unused_variables)]

pub struct School {
    grades: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School{ grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let current_grade = self.grades.entry(grade).or_insert(Vec::new());
        current_grade.push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self.grades.keys().map(|key| *key).collect::<Vec<u32>>();

        result.sort();
        result
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if self.grades.contains_key(&grade) {
            let mut result = (*self.grades.get(&grade).unwrap()).clone();
            result.sort();

            Some(result)
        } else {
            None
        }
    }
}
