package containers;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
// import org.hamcrest.core.IsNull;

import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

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
        LinkedList<Integer> ll = new LinkedList<>();

        assertThat(ll, is(not(nullValue())));
        assertThat(ll.size(), is(0));

        Iterator it = ll.iterator();
        assertThat(it.hasNext(), is(false));
    }

    @Test
    public void testAddSimple()
    {
        LinkedList<Integer> ll = new LinkedList<>();

        assertThat(ll.size(), is(0));

        ll.add(Integer.valueOf(1));
        assertThat(ll.size(), is(1));

        ll.add(Integer.valueOf(1));
        assertThat(ll.size(), is(2));

        ll.add(Integer.valueOf(2));
        assertThat(ll.size(), is(3));
    }

    @Test
    public void testAddButBetter()
    {
        LinkedList<Integer> ll = new LinkedList<>();

        assertThat(ll.size(), is(0));

        ll.add(1);
        ll.add(1);
        ll.add(2);
        ll.add(3);
        ll.add(5);
        ll.add(8);
        ll.add(13);

        assertThat(ll.size(), is(7));

        Iterator<Integer> it = ll.iterator();
        assertThat(it.hasNext(), is(true));

        assertThat(it.next(), is(1));
        assertThat(it.next(), is(1));
        assertThat(it.next(), is(2));
        assertThat(it.next(), is(3));
        assertThat(it.next(), is(5));
        assertThat(it.next(), is(8));
        assertThat(it.next(), is(13));

        assertThat(it.hasNext(), is(false));
    }

    @Test
    public void testAddAll()
    {
        LinkedList<Integer> ll = new LinkedList<>();

        assertThat(ll.size(), is(0));
        assertThat(ll.isEmpty(), is(true));

        List<Integer> numbers = Arrays.asList(1, 1, 2, 3, 5, 8, 13);
        ll.addAll(numbers);

        assertThat(ll.size(), is(numbers.size()));
        assertThat(ll.isEmpty(), is(not(true)));

        Iterator<Integer> it = ll.iterator();
        assertThat(it.hasNext(), is(true));

        assertThat(it.next(), is(1));
        assertThat(it.next(), is(1));
        assertThat(it.next(), is(2));
        assertThat(it.next(), is(3));
        assertThat(it.next(), is(5));
        assertThat(it.next(), is(8));
        assertThat(it.next(), is(13));

        assertThat(it.hasNext(), is(false));
    }

    @Test
    public void testClone()
    {
        LinkedList<Integer> original = new LinkedList<>();

        for (int i = 1; i < 5; i++) {
            original.add(i);
        }

        LinkedList<Integer> copy = original.clone();

        assertThat(copy, is(not(sameInstance(original))));
        assertThat(copy.size(), is(4));

        Iterator<Integer> it = copy.iterator();

        assertThat(it.next(), is(1));
        assertThat(it.next(), is(2));
        assertThat(it.next(), is(3));
        assertThat(it.next(), is(4));

        assertThat(it.hasNext(), is(false));
    }

    @Test
    public void testContains()
    {
        LinkedList<Integer> ll = new LinkedList<>();

        List<Integer> numbers = Arrays.asList(1, 1, 2, 3, 5, 8, 13);
        ll.addAll(numbers);

        assertThat(ll.size(), is(numbers.size()));
        assertThat(ll.isEmpty(), is(not(true)));

        assertThat(ll.contains(1), is(true));
        assertThat(ll.contains(2), is(true));
        assertThat(ll.contains(5), is(true));
        assertThat(ll.contains(8), is(true));
        assertThat(ll.contains(13), is(true));

        assertThat(ll.contains(337), is(not(true)));
    }

    @Test
    public void testContainsAll()
    {
        LinkedList<Integer> ll = new LinkedList<>();

        List<Integer> numbers = Arrays.asList(1, 1, 2, 3, 5, 8, 13);
        ll.addAll(numbers);

        assertThat(ll.size(), is(numbers.size()));
        assertThat(ll.isEmpty(), is(not(true)));

        assertThat(ll.containsAll(numbers), is(true));

        List<Integer> otherNumbers = Arrays.asList(0, 1, 1, 2, 3, 5, 8, 13);
        assertThat(ll.containsAll(otherNumbers), is(not(true)));
    }
}
