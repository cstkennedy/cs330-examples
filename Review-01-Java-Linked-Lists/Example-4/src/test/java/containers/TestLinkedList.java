package containers;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
// import org.hamcrest.core.IsNull;

import java.util.Iterator;

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
public class TestLinkedList
{
    @Test
    public void testDefaultConstructor()
    {
        LinkedList ll = new LinkedList();

        assertThat(ll, is(not(nullValue())));
        assertThat(ll.size(), is(0));
    }

    @Test
    public void testAddSimple()
    {
        LinkedList ll = new LinkedList();

        assertThat(ll.size(), is(0));

        ll.add(1);
        assertThat(ll.size(), is(1));

        ll.add(1);
        assertThat(ll.size(), is(2));

        ll.add(2);
        assertThat(ll.size(), is(3));
    }

    @Test
    public void testClone()
    {
        LinkedList original = new LinkedList();

        for (int i = 1; i < 5; i++) {
            original.add(i);
        }

        LinkedList copy = original.clone();

        assertThat(copy, is(not(sameInstance(original))));
        assertThat(copy.size(), is(4));
    }

}
