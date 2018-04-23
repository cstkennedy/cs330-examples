package edu.odu.cs.cs350.examples;

public class Student implements Cloneable {
    /**
     * Constant defining the default student name
     */
    public static final String DEFAULT_NAME = "John Q. Smith";

    private String name;

    /**
     * Create a Student with the default name
     * of John Q. Smith
     */
    public Student()
    {
        this.name = DEFAULT_NAME;
    }

    /**
     * Create a Student with a specified name
     *
     * @param n desired name
     */
    public Student(String n)
    {
        this.name = n;
    }

    /**
     * Get the Name
     */
    public String getName()
    {
        return name;
    }

    /**
     * Change the student name
     *
     * @param n desired name
     */
    public void setName(String n)
    {
        name = n;
    }

    /**
     * Compare 2 `Student`s based on name
     *
     * @param rhs the other (right-hand-side) student object
     */
    @Override
    public boolean equals(Object rhs)
    {
        if(!(rhs instanceof Student)) {
            return false;
        }

        return (this.name).equals(((Student)rhs).name);
    }

    /**
     * Return a hashcode
     */
    @Override
    public int hashCode()
    {
        return name.hashCode();
    }

    /**
     * Return a (deep) copy of this object.
     */
    @Override
    public Object clone()
    {
        return new Student(name);
    }

    /**
     *
     */
    @Override
    public String toString()
    {
        return name;
    }
}