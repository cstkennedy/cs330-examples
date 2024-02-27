use std::collections::HashSet;
use std::fmt;

use crate::student::Student;


const DEFAULT_MAX_STUDENTS: usize = 10;

pub struct Roster {
    pub course_num: &'static str,
    pub enroll_limit: usize,
    pub students: HashSet<Student>
}

impl Roster {
    pub fn new(limit: usize, num: &'static str) -> Self {
        Roster {
            course_num: num,
            enroll_limit: limit,
            students: HashSet::new(),
        }
    }

    pub fn enroll(&mut self, stu: Student) -> bool {
        if self.students.len() == self.enroll_limit {
            return false;
        }

        if self.students.contains(&stu) {
            return false;
        }

        self.students.insert(stu);
        return true;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Student> {
        self.students.iter()
    }
}

impl fmt::Display for Roster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.course_num)?;

        for stu in self.students.iter() {
            writeln!(f, "  - {}", stu)?;
        }
        Ok(())
    }
}
