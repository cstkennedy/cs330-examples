package edu.odu.cs.cs350.examples;

import java.util.Set;
import java.util.LinkedHashSet;

/**
 * A class roster listing all students enrolled
 * in a course
 */
public class Roster implements Cloneable {
    public static final int DEFAULT_MAX_STUDENTS = 10;

    private String courseNum;
    private int    enrollLimit;

    private LinkedHashSet<Student> students;

    /**
     * Create a class roster with a limit
     * of DEFAULT_MAX_STUDENTS `Student`s
     * and the course number set to
     * "CS 150"
     */
    public Roster()
    {
        this.courseNum   = "CS 150";
        this.enrollLimit = DEFAULT_MAX_STUDENTS;

        students = new LinkedHashSet<Student>();
    }

    /**
     * Create a class roster with a specified 
     * enrollment limit and course number
     *
     * @param l enrollment limit
     * @param c course number 
     */
    public Roster(int l, String c)
    {
        this.courseNum   = c;
        this.enrollLimit = l;

        students = new LinkedHashSet<Student>();
    }

    /**
     * Retrieve the course number
     */
    public String getCourseNum()
    {
        return this.courseNum;
    }

    /**
     * Change the course number
     *
     * @param n desired course number
     */
    public void setCourseNum(String n)
    {
        this.courseNum = n;
    }

    /**
     * Retrieve the enrollment limit
     */
    public int getEnrollLimit()
    {
        return this.enrollLimit;
    }

    /**
     * Change the enrollment limit
     *
     * @param n desired limit
     */
    public void setEnrollLimit(int n)
    {
        this.enrollLimit = n;
    }

    /**
     * Attempt to enroll a Student in the course
     *
     * Rules: 
     *   1- A student can not be added if the enrollment
     *     limit has been reached.
     *   2- A can not be added to a roster multiple times 
     *
     * @param stu Student to enroll
     *
     * @return true if the Student was successfully enrolled
     *    in the course (added to the roster)
     */
    public boolean enroll(Student stu)
    {
        // limit has been reached
        if(students.size() == enrollLimit) {
            return false;
        }

        // Student was previously added to the roster
        if(students.contains(stu)) {
            return false;
        }

        students.add(stu);
        return true;
    }

    /**
     * Retrieve the number of enrolled students
     */
    public int numEnrolled()
    {
        return students.size();
    }

    /**
     * Return a collection of students in
     * the order they were enrolled (added)
     *
     * @return Set of enrolled students. If no students
     *     have been added to the roster, the set will be 
     *     empty.
     */
    public Set<Student> listEnrolledStudents()
    {
        return this.students;
    }

    /**
     * Compare 2 `Student`s based on name
     *
     * @param rhs the other (right-hand-side) student object
     */
    @Override
    public boolean equals(Object rhs)
    {
        if(!(rhs instanceof Roster)) {
            return false;
        }

        Roster rhsRoster = (Roster) rhs;

        if (!this.courseNum.equals(rhsRoster.courseNum)) {
            return false;
        }

        if (this.enrollLimit != rhsRoster.enrollLimit) {
            return false;
        }

        if (!this.students.equals(rhsRoster.students)) {
            return false;
        }        

        return true;
    }

    /**
     * Return a hashcode
     */
    @Override
    public int hashCode()
    {
        int hc = courseNum.hashCode();

        hc += enrollLimit;
        hc += students.hashCode();

        return hc;
    }

    /**
     * Return a (deep) copy of this object.
     */
    @Override
    public Object clone()
    {
        Roster cpy = new Roster(this.enrollLimit, this.courseNum);

        // Now add the students
        for (Student stu : this.students) {
            cpy.students.add(stu);
        }

        //return new Student(name);
        return cpy;
    }

    /**
     * Generate a String containing the course number,
     * number of enrolled students, enrollment limit, and the names
     * of all enrolled students.
     */
    @Override
    public String toString()
    {
        StringBuilder bld = new StringBuilder();

        bld.append(this.courseNum);
        bld.append(
            String.format(" -> %2d of %2d (%4.2f%% full)\n",
                          students.size(),
                          enrollLimit,
                          100.0 * students.size() / enrollLimit)
        );

        for (Student s : students) {
            bld.append("  -" + s + "\n");
        }

        return bld.toString();
    }
}