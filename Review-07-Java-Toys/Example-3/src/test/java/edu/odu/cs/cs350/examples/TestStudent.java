package edu.odu.cs.cs350.examples;

import org.junit.FixMethodOrder;
import org.junit.runners.MethodSorters;
import org.junit.Test;
import org.junit.Before;

import static org.junit.Assert.*;

import static org.hamcrest.CoreMatchers.*;
import org.hamcrest.core.IsNull;

/**
 * 1 - Does this piece of code perform the operations 
 *     it was designed to perform?
 * 
 * 2 - Does this piece of code do something it was not 
 *     designed to perform?
 * 
 * 1 Test per mutator
 */
@FixMethodOrder(MethodSorters.NAME_ASCENDING)
public class TestStudent {

    Student student;

    @Before
    public void setUp()
    {
        student = new Student();
    }

    @Test
    public void testDefaultConstructor()
    {      
        assertThat(student.getName(), is(Student.DEFAULT_NAME));
        assertThat(student.getName(), equalTo(Student.DEFAULT_NAME));

        assertThat(student.hashCode(),
                   equalTo((Student.DEFAULT_NAME).hashCode()));
        
        assertThat(student.toString(), equalTo(Student.DEFAULT_NAME));
    }

    @Test
    public void testNonDefaultConstructor()
    {
        String  desiredName = "Tommy Oliver";
        Student tommy       = new Student(desiredName);

        assertThat(tommy.getName(), equalTo(desiredName));

        assertThat(tommy.hashCode(),
                   not(equalTo(student.hashCode())));
        
        assertThat(tommy.toString(), equalTo(desiredName));
        assertThat(tommy.toString(), containsString(desiredName));

        assertThat(tommy, not(equalTo(student)));
    }

    @Test
    public void testSetName()
    {
        Student john    = new Student();
        String  newName = "John T. Smith";

        int oldHashCode = john.hashCode();

        john.setName(newName);

        assertThat(john.getName(), equalTo(newName));
        assertThat(john.hashCode(), not(oldHashCode));
        
        //assertThat(john.toString(), equalTo(newName));
        assertThat(john.toString(), containsString(newName));

        assertThat(john, not(equalTo(student)));
    }

    @Test
    public void testClone()
    {
        fail("Left as an Exercise");
    }


}