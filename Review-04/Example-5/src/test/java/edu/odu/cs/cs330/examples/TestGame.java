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
public class TestGame
{
    Player tom;
    Player aCylon;

    Board emptyBoard;

    Game aGame;

    @Before
    public void setUp()
    {
        tom       = new Player("Tom");
        aCylon    = new Player();

        emptyBoard = new Board();

        aGame = new Game(tom, aCylon);
    }

    @Test
    public void testConstructor()
    {
        assertThat(aGame.getPlayer1(), equalTo(tom));
        assertThat(aGame.getPlayer2(), equalTo(aCylon));

        assertThat(aGame.getPlayer1().getSymbol(), is('X'));
        assertThat(aGame.getPlayer2().getSymbol(), is('O'));

        assertThat(aGame.isOver(), is(false));

        assertThat(aGame.getWinner(), is(nullValue()));
        assertThat(aGame.getLoser(), is(nullValue()));

        // Can not test without Board.equals method
        assertThat(aGame.getBoard(), equalTo(emptyBoard));
    }

    @Test
    public void testPlayRound()
    {
        // Can not test due to hardcoded System.in use in Player.nextMove
        fail("Can not test");
    }
}
