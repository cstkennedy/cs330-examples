use enroll_students::roster::Roster;
use enroll_students::student::Student;

fn main() {
    let john = Student::new("John");
    let tom = Student::new("Tom");
    let jay = Student::new("Jay");
    let oscar = Student::new("Oscar");

    let all_students = [john, tom, jay, oscar];

    println!("{}", tom);

    let mut cs330 = Roster::new(3, "CS 330");

    for stu in all_students.into_iter() {
        let course_num = cs330.course_num;

        let name = stu.name;

        if cs330.enroll(stu) {
            println!("{name} enrolled in {course_num}");
        } else {
            println!("{name} NOT enrolled in {course_num}");
        }
    }

    println!();
    println!("{cs330}");
}
