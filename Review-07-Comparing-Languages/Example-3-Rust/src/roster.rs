use std::collections::HashSet;
use std::fmt;

use crate::student::Student;

const DEFAULT_MAX_STUDENTS: usize = 10;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Roster<'a> {
    pub course_num: &'a str,
    pub enroll_limit: usize,
    students: HashSet<Student<'a>>,
}

impl<'a> Default for Roster<'a> {
    fn default() -> Self {
        Roster {
            course_num: "CS 150",
            enroll_limit: DEFAULT_MAX_STUDENTS,
            students: Default::default(),
        }
    }
}

impl<'a> Roster<'a> {
    pub fn new(limit: usize, num: &'a str) -> Self {
        Roster {
            course_num: num,
            enroll_limit: limit,
            students: HashSet::new(),
        }
    }

    pub fn enroll(&mut self, stu: Student<'a>) -> bool {
        if self.students.len() == self.enroll_limit {
            return false;
        }

        if self.students.contains(&stu) {
            return false;
        }

        self.students.insert(stu);
        return true;
    }

    pub fn list_enrolled_students(&self) -> &HashSet<Student<'a>> {
        &self.students
    }

    pub fn num_enrolled(&self) -> usize {
        self.students.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Student> {
        self.students.iter()
    }
}

impl<'a> fmt::Display for Roster<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let percent_full = 100.0 * (self.num_enrolled() / self.enroll_limit) as f64;

        writeln!(
            f,
            "{} -> {} of {} ({}% full)",
            self.course_num,
            self.num_enrolled(),
            self.enroll_limit,
            percent_full
        )?;

        for stu in self.students.iter() {
            writeln!(f, "  - {}", stu)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_default_constructor() {
        let default_course = Roster::default();

        assert_that!(default_course.course_num, equal_to("CS 150"));
        assert_that!(default_course.enroll_limit, equal_to(DEFAULT_MAX_STUDENTS));

        assert_that!(default_course.num_enrolled(), equal_to(0));

        // No students have been added
        assert_that!(default_course.list_enrolled_students().len(), equal_to(0));

        // skipping hashcode
        // skipping equals

        // test __str__
        let default_course_str = default_course.to_string();
        assert_that!(default_course_str.contains("CS 150"), is(true));
        assert_that!(
            default_course_str.contains(&default_course.num_enrolled().to_string()),
            is(true)
        );
        assert_that!(
            default_course_str.contains(&DEFAULT_MAX_STUDENTS.to_string()),
            is(true)
        );
    }

    /*
    def test_non_default_constructor(default_courses, students_and_lists):
        _, _, _, _, all_students, first_three_students = students_and_lists
        default_course, empty_cs350 = default_courses

        assert_that!(empty_cs350.course_num, equal_to("CS 350"))
        assert_that!(empty_cs350.enroll_limit, equal_to(3))

        assert_that!(default_course.num_enrolled(), equal_to(0))

        // No students have been added
        assert_that!(empty_cs350.list_enrolled_students(), not_none())
        assert_that!(len(empty_cs350.list_enrolled_students()), equal_to(0))

        // NOT skipping hashcode
        assert_that!(hash(empty_cs350), is_not(equal_to(hash(default_course))))
        // NOT skipping equals
        assert_that!(empty_cs350, is_not(equal_to(default_course)))

        // test __str__
        assert_that!(str(empty_cs350), contains_string("CS 350"))
        assert_that!(str(empty_cs350), contains_string(f"{empty_cs350.num_enrolled()}"))
        assert_that!(str(empty_cs350), contains_string("3"))


    def test_set_course_num(default_courses, students_and_lists):
        _, _, _, _, all_students, first_three_students = students_and_lists
        default_course, empty_cs350 = default_courses

        cs252 = Roster()

        old_hash_code = hash(cs252)

        cs252.course_num = "CS 252"

        assert_that!(cs252.course_num, equal_to("CS 252"))
        assert_that!(cs252.enroll_limit, equal_to(DEFAULT_MAX_STUDENTS))

        assert_that!(default_course.num_enrolled(), equal_to(0))

        // No students have been added
        assert_that!(cs252.list_enrolled_students(), not_none())
        assert_that!(len(cs252.list_enrolled_students()), equal_to(0))

        // NOT skipping hashcode
        assert_that!(hash(cs252), is_not(equal_to(old_hash_code)))
        // NOT skipping equals
        assert_that!(cs252, is_not(equal_to(default_course)))

        // test __str__
        assert_that!(str(cs252), contains_string("CS 252"))
        assert_that!(str(cs252), contains_string(f"{cs252.num_enrolled()}"))  // fixed mistake
        assert_that!(str(cs252), contains_string(f"{DEFAULT_MAX_STUDENTS}"))


    def test_set_enroll_limit(default_courses):
        default_course, empty_cs350 = default_courses

        empty_cs350.enroll_limit = 2

        assert_that!(empty_cs350.course_num, equal_to("CS 350"))
        assert_that!(empty_cs350.enroll_limit, equal_to(2))

        assert_that!(default_course.num_enrolled(), equal_to(0))

        // No students have been added
        assert_that!(empty_cs350.list_enrolled_students(), not_none())
        assert_that!(len(empty_cs350.list_enrolled_students()), equal_to(0))

        // NOT skipping hashcode
        assert_that!(hash(empty_cs350), is_not(equal_to(hash(default_course))))
        // NOT skipping equals
        assert_that!(empty_cs350, is_not(equal_to(default_course)))

        // test __str__
        assert_that!(str(empty_cs350), contains_string("CS 350"))
        assert_that!(str(empty_cs350), contains_string(f"{empty_cs350.num_enrolled()}"))
        assert_that!(str(empty_cs350), contains_string("2"))


    def test_enroll(default_courses, students_and_lists):
        john, tom, jay, oscar, all_students, first_three_students = students_and_lists
        default_course, _ = default_courses

        // This is where the fun starts
        cs725 = Roster(3, "CS 725")

        old_hash_code = hash(cs725)

        // try to add 4 students
        assert_that!(cs725.enroll(john), is_(True))
        assert_that!(cs725.enroll(tom), is_(True))
        assert_that!(cs725.enroll(jay), is_(True))
        assert_that!(cs725.enroll(oscar), is_(False))  // should fail (limit of 3)

        assert_that!(cs725.course_num, equal_to("CS 725"))
        assert_that!(cs725.enroll_limit, equal_to(3))
        assert_that!(cs725.num_enrolled(), equal_to(3))  // fixed mistake

        // 3 students have been added
        assert_that!(cs725.list_enrolled_students(), not_none())
        assert_that!(len(cs725.list_enrolled_students()), equal_to(3))
        assert_that!(
            cs725.list_enrolled_students(),
            contains_inanyorder(*first_three_students),
        )

        assert_that!(hash(cs725), is_not(equal_to(old_hash_code)))

        old_hash_code = hash(cs725)

        // Change the limit to 4
        cs725.set_enroll_limit(4)  // mistake - this should be cs725
        assert_that!(cs725.enroll_limit, equal_to(4))  // mistake - this should be cs725

        // try to add a 4th student with the new limit of 4
        assert_that!(cs725.enroll(oscar), is_(True))  // should succeed (limit of 4)

        // 4 students have been added
        assert_that!(cs725.list_enrolled_students(), not_none())
        assert_that!(len(cs725.list_enrolled_students()), equal_to(4))
        assert_that!(
            cs725.list_enrolled_students(),
            contains_inanyorder(*all_students),
        )

        // NOT skipping hashcode
        assert_that!(hash(cs725), is_not(equal_to(old_hash_code)))
        // NOT skipping equals
        assert_that!(cs725, is_not(equal_to(default_course)))

        // **test flaw** - did not exercise adding the same student twice
        assert_that!(cs725.enroll(tom), is_(False))  // should fail

        // test __str__
        assert_that!(str(cs725), contains_string("CS 725"))
        assert_that!(str(cs725), contains_string(f"{cs725.num_enrolled()}"))
        assert_that!(str(cs725), contains_string("4"))

        assert_that!(str(cs725), contains_string(str(all_students[0])))
        assert_that!(str(cs725), contains_string(str(all_students[1])))
        assert_that!(str(cs725), contains_string(str(all_students[2])))
        assert_that!(str(cs725), contains_string(str(all_students[3])))


    def test_clone(default_courses, students_and_lists):
        john, tom, jay, oscar, all_students, first_three_students = students_and_lists
        default_course, _ = default_courses

        // This is where the fun continues
        cs725 = Roster(3, "CS 725")

        cs725.enroll(john)
        cs725.enroll(tom)
        cs725.enroll(jay)

        // Make the copy
        copy_cs725 = copy.deepcopy(cs725)

        // Both Rosters should still be identical
        assert_that!(copy_cs725.course_num, equal_to(cs725.course_num))
        assert_that!(copy_cs725.enroll_limit, equal_to(cs725.enroll_limit))
        assert_that!(copy_cs725.num_enrolled(), equal_to(cs725.num_enrolled()))
        assert_that!(hash(copy_cs725), equal_to(hash(cs725)))
        assert_that!(copy_cs725, equal_to(cs725))
        assert_that!(str(copy_cs725), equal_to(str(cs725)))

        // But distinct
        assert_that!(copy_cs725, is_not(same_instance(cs725)))
        assert_that!(
            copy_cs725.list_enrolled_students(),
            is_not(same_instance(cs725.list_enrolled_students())),
        )

        // Change the limit to 4
        copy_cs725.set_enroll_limit(4)
        assert_that!(copy_cs725.enroll_limit, equal_to(4))
        assert_that!(copy_cs725.enroll(oscar), is_(True))

        assert_that!(copy_cs725.list_enrolled_students(), not_none())
        assert_that!(len(copy_cs725.list_enrolled_students()), equal_to(4))
        assert_that!(
            copy_cs725.list_enrolled_students(),
            contains_inanyorder(*all_students),
        )

        assert_that!(hash(copy_cs725), is_not(equal_to(hash(cs725))))
        assert_that!(copy_cs725, is_not(equal_to(default_course)))
        assert_that!(copy_cs725, is_not(equal_to(cs725)))

        // cs725 should be unchanged
        assert_that!(len(cs725.list_enrolled_students()), equal_to(3))
        assert_that!(
            cs725.list_enrolled_students(),
            contains_inanyorder(*first_three_students),
        )

        // test __str__
        assert_that!(str(copy_cs725), contains_string("CS 725"))
        assert_that!(str(copy_cs725), contains_string(f"{copy_cs725.num_enrolled()}"))
        assert_that!(str(copy_cs725), contains_string("4"))

        assert_that!(str(copy_cs725), contains_string(str(all_students[0])))
        assert_that!(str(copy_cs725), contains_string(str(all_students[1])))
        assert_that!(str(copy_cs725), contains_string(str(all_students[2])))
        assert_that!(str(copy_cs725), contains_string(str(all_students[3])))

        assert_that!(str(copy_cs725), is_not(str(equal_to(cs725))))
    */
}
