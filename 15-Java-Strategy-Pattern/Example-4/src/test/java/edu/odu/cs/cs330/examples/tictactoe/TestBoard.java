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
public class TestBoard
{
    private static final String EXPECTED_EMPTY_STRING = "1|2|3\n4|5|6\n7|8|9\n";

    Board aBoard;

    @BeforeEach
    public void setUp()
    {
        aBoard = new Board();
    }

    @Test
    public void testDefaultConstructor()
    {
        for (int i = 1; i <= 9; i++) {
            char expectedChar = Character.forDigit(i, 10);
            assertThat(aBoard.getCell(i), is(expectedChar));
        }

        assertThat(aBoard.toString(), equalTo(EXPECTED_EMPTY_STRING));
        assertFalse(aBoard.isFull());

        char[] retrieved = aBoard.get3Cells(1, 2, 3);

        assertThat(retrieved[0], is(equalTo('1')));
        assertThat(retrieved[1], is(equalTo('2')));
        assertThat(retrieved[2], is(equalTo('3')));
    }

    @Test
    public void testSetCell()
    {
        aBoard.setCell(1, 'X');
        aBoard.setCell(9, 'O');

        assertThat(aBoard.getCell(1), is('X'));
        assertThat(aBoard.getCell(9), is('O'));

        char[] retrieved = aBoard.get3Cells(1, 5, 9);

        assertThat(retrieved[0], is(equalTo('X')));
        assertThat(retrieved[1], is(equalTo('5')));
        assertThat(retrieved[2], is(equalTo('O')));

        assertThat(aBoard.toString(), not(equalTo(EXPECTED_EMPTY_STRING)));
        assertThat(aBoard.toString(), equalTo("X|2|3\n4|5|6\n7|8|O\n"));

        assertFalse(aBoard.isFull());
    }

}
