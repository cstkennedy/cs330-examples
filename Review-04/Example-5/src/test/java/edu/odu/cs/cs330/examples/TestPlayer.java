package edu.odu.cs.cs330.examples;

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
public class TestPlayer
{
    @Test
    public void testPlayerDefaultConstructor()
    {
        Player tom = new Player("Tom");

        assertEquals("Tom", tom.toString());

        // Fancy!!
        assertThat(tom.toString(), equalTo("Tom"));
    }

    // Where are the rest of the tests? This might
    // be worth an F... if I am lucky.

}