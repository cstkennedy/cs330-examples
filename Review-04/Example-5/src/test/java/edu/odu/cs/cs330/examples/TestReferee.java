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
        assertThat(aReferee.checkForWin(), is(0));

        for (int i = 1; i < 10; i++) {
            assertThat(aReferee.selectedCellIsEmpty(i), is(true));
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

        assertThat(hReferee.selectedCellIsEmpty(4), is(false));
        assertThat(hReferee.selectedCellIsEmpty(5), is(false));
        assertThat(hReferee.selectedCellIsEmpty(6), is(false));

        assertThat(hReferee.checkForWin(), is(1));
    }

    @Test
    public void checkForVerticalWin()
    {
        Board vBoard = new Board();

        vBoard.setCell(2, 'O');
        vBoard.setCell(5, 'O');
        vBoard.setCell(8, 'O');

        Referee vReferee = new Referee(vBoard);

        assertThat(vReferee.selectedCellIsEmpty(2), is(false));
        assertThat(vReferee.selectedCellIsEmpty(5), is(false));
        assertThat(vReferee.selectedCellIsEmpty(8), is(false));

        assertThat(vReferee.checkForWin(), is(2));
    }

    @Test
    public void checkForDiagonalWin()
    {
        Board dBoard = new Board();

        dBoard.setCell(3, 'O');
        dBoard.setCell(5, 'O');
        dBoard.setCell(7, 'O');

        Referee dReferee = new Referee(dBoard);

        assertThat(dReferee.selectedCellIsEmpty(3), is(false));
        assertThat(dReferee.selectedCellIsEmpty(5), is(false));
        assertThat(dReferee.selectedCellIsEmpty(7), is(false));

        assertThat(dReferee.checkForWin(), is(2));
    }
}
