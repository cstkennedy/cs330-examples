use enroll_students::student::Student;
use enroll_students::roster::Roster;

fn main() {
    let john = Student::new("John");
    let tom = Student::new("Tom");
    let jay = Student::new("Jay");
    let oscar = Student::new("Oscar");

    let all_students = [john, tom, jay, oscar];

    let mut cs330 = Roster::new(3, "CS 330");

    for stu in all_students.iter() {
        let course_num = cs330.course_num;

        if cs330.enroll(stu.clone()) {
            println!("{stu} enrolled in {course_num}");
        }
        else {
            println!("{stu} NOT enrolled in {course_num}");
        }
    }

    println!();
    println!("{cs330}");
}
