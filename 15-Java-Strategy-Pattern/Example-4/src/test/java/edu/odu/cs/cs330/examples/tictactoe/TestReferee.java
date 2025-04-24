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
public class TestReferee
{
    Board   emptyBoard;
    Referee aReferee;

    @BeforeEach
    public void setUp()
    {
        emptyBoard = new Board();

        aReferee = new Referee(emptyBoard);
    }

    @Test
    public void testConstructor()
    {
        assertThat(aReferee.checkForWin(), is(not(oneOf('X', 'O'))));

        for (int i = 1; i < 10; i++) {
            assertThat(aReferee.saysMoveIsValid(i), is(true));
        }
    }

    @Test
    public void checkForHorizontalWin()
    {
        Board hBoard = new Board();

        hBoard.setCell(4, 'X');
        hBoard.setCell(5, 'X');
        hBoard.setCell(6, 'X');

        Referee hReferee = new Referee(hBoard);

        assertThat(hReferee.saysMoveIsValid(4), is(false));
        assertThat(hReferee.saysMoveIsValid(5), is(false));
        assertThat(hReferee.saysMoveIsValid(6), is(false));

        assertThat(hReferee.checkForWin(), is(true));
    }

    @Test
    public void checkForVerticalWin()
    {
        Board vBoard = new Board();

        vBoard.setCell(2, 'O');
        vBoard.setCell(5, 'O');
        vBoard.setCell(8, 'O');

        Referee vReferee = new Referee(vBoard);

        assertThat(vReferee.saysMoveIsValid(2), is(false));
        assertThat(vReferee.saysMoveIsValid(5), is(false));
        assertThat(vReferee.saysMoveIsValid(8), is(false));

        assertThat(vReferee.checkForWin(), is(true));
    }

    @Test
    public void checkForDiagonalWin()
    {
        Board dBoard = new Board();

        dBoard.setCell(3, 'O');
        dBoard.setCell(5, 'O');
        dBoard.setCell(7, 'O');

        Referee dReferee = new Referee(dBoard);

        assertThat(dReferee.saysMoveIsValid(3), is(false));
        assertThat(dReferee.saysMoveIsValid(5), is(false));
        assertThat(dReferee.saysMoveIsValid(7), is(false));

        assertThat(dReferee.checkForWin(), is(true));
    }
}
