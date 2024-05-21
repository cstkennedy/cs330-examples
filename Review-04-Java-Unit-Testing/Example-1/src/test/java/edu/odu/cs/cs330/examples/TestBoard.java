package edu.odu.cs.cs330.examples;

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
    final String expectedEmptyStr = "1|2|3\n4|5|6\n7|8|9\n";

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

        assertThat(aBoard.toString(), equalTo(expectedEmptyStr));
        assertFalse(aBoard.isFull());

        Board.Pair[] retrieved = aBoard.get3Cells(1, 2, 3);
        Board.Pair[] expected = {
            new Board.Pair(0, '1'),
            new Board.Pair(1, '2'),
            new Board.Pair(2, '3')
        };

        assertThat(retrieved[0], equalTo(expected[0]));
        assertThat(retrieved[1], equalTo(expected[1]));
        assertThat(retrieved[2], equalTo(expected[2]));
    }

    @Test
    public void testSetCell()
    {
        aBoard.setCell(1, 'X');
        aBoard.setCell(9, 'O');

        assertThat(aBoard.getCell(1), is('X'));
        assertThat(aBoard.getCell(9), is('O'));

        Board.Pair[] retrieved = aBoard.get3Cells(1, 5, 9);
        Board.Pair[] expected = {
            new Board.Pair(0, 'X'),
            new Board.Pair(4, '5'),
            new Board.Pair(8, 'O')
        };

        assertThat(retrieved[0], equalTo(expected[0]));
        assertThat(retrieved[1], equalTo(expected[1]));
        assertThat(retrieved[2], equalTo(expected[2]));

        assertThat(aBoard.toString(), not(equalTo(expectedEmptyStr)));
        assertThat(aBoard.toString(), equalTo("X|2|3\n4|5|6\n7|8|O\n"));

        assertFalse(aBoard.isFull());
    }

}
