import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
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
@TestMethodOrder(MethodOrderer.MethodName.class)
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
