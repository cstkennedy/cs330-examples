package edu.odu.cs.cs330.examples;

import edu.odu.cs.cs350.examples.Student;
import edu.odu.cs.cs350.examples.Roster;

class EnrollStudents {
    /**
     * This is a non-trivial main function 
     */
    public static void main(String args[])
    {
        Student john  = new Student("John");
        Student tom   = new Student("Tom");
        Student jay   = new Student("Jay");
        Student oscar = new Student("Oscar");

        Student[] allStudents = {john, tom, jay, oscar};

        Roster cs330 = new Roster(3, "CS 330");

        for (Student s : allStudents) {

            if(cs330.enroll(s)) {
                System.out.println(s + " enrolled in " + cs330.getCourseNum());
            }
            else {
                System.out.println(s + " NOT enrolled in " + cs330.getCourseNum());
            }
        }

        System.out.println();

        System.out.println(cs330);
    }
}