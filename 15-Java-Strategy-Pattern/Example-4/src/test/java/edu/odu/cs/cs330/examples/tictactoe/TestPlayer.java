package edu.odu.cs.cs330.examples.tictactoe;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
// import org.hamcrest.core.IsNull;

/**
 * 1 - Does this piece of code perform the operations
 *     it was designed to perform?
 *
 * 2 - Does this piece of code do something it was not
 *     designed to perform?
 *
 * 1 Test per mutator
 */
@TestMethodOrder(MethodOrderer.MethodName.class)
public class TestPlayer
{
    Player tom;
    Player aCylon;
    Player theDoctor;

    @BeforeEach
    public void setUp()
    {
        tom       = new Player("Tom", null);
        aCylon    = Player.REFERENCE_CYLON.clone();
        theDoctor = new Player("The Doctor", null);
    }

    @Test
    public void testPlayerDefaultConstructor()
    {
        assertTrue(Player.isGeneric(aCylon));

        assertThat(aCylon.hashCode(), is(not(tom.hashCode())));
        assertThat(aCylon, not(equalTo(tom)));

        // Hand wave... These are not the cylons you are looking for.
        assertThat(aCylon.isHuman(), is(true));
        assertThat(aCylon.isComputer(), is(false));
    }

    @Test
    public void testPlayerConstructor()
    {
        assertEquals("Tom", tom.toString());
        assertThat(tom.toString(), equalTo("Tom"));

        assertThat(tom.hashCode(), is(not(theDoctor.hashCode())));
        assertThat(tom, not(equalTo(theDoctor)));

        assertThat(tom.isHuman(), is(true));
        assertThat(tom.isComputer(), is(false));
    }

    @Test
    public void testSetName()
    {
        int oldHashCode = theDoctor.hashCode();

        assertThat(theDoctor.getName(), is("The Doctor"));
        assertThat(theDoctor.hashCode(), is(oldHashCode));

        theDoctor.setName("David Tennant");
        assertThat(theDoctor.getName(), is("David Tennant"));
        assertThat(theDoctor.hashCode(), is(not(oldHashCode)));

        theDoctor.setName("Mat Smith");
        assertThat(theDoctor.getName(), is("Mat Smith"));
        assertThat(theDoctor.hashCode(), is(not(oldHashCode)));

        theDoctor.setName("Peter Capaldi");
        assertThat(theDoctor.getName(), is("Peter Capaldi"));
        assertThat(theDoctor.hashCode(), is(not(oldHashCode)));

        theDoctor.setName("Jodie Whittaker");
        assertThat(theDoctor.getName(), is("Jodie Whittaker"));
        assertThat(theDoctor.hashCode(), is(not(oldHashCode)));

        // No clone function, can't test equals
    }

    @Test
    public void testClone()
    {
        Player theOriginal = theDoctor.clone();

        assertThat(theDoctor.hashCode(), equalTo(theOriginal.hashCode()));
        assertThat(theDoctor, equalTo(theOriginal));

        theOriginal.setName("William Hartnell");
        assertThat(theDoctor.hashCode(), not(equalTo(theOriginal.hashCode())));
        assertThat(theDoctor, not(equalTo(theOriginal)));
    }

    @Test
    public void testNextMove()
    {
        // Can not test due to hardcoded System.in use in Player.nextMove
        fail("Can not test");
    }
}
