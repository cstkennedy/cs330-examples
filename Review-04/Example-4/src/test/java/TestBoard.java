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
public class TestBoard
{
    @Test
    public void testBoardButPoorly()
    {
        Board board = new Board();

        String expectedInitial = "1|2|3\n4|5|6\n7|8|9\n";
        String expectedFinal   = "X|X|X\nX|X|X\nX|X|X\n";

        assertThat(board.toString(), equalTo(expectedInitial));
        assertFalse(board.isFull());

        for (int i = 0 ; i < 9; i++) {
            board.setCell((i + 1), 'X');
        }

        assertThat(board.toString(), equalTo(expectedFinal));
        assertTrue(board.isFull());
    }

    // Where are the rest of the tests? This might
    // be worth an F... if I am lucky.

}
