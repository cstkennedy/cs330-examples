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
public class TestGame
{
    Player tom;
    Player aCylon;

    Board emptyBoard;

    Game aGame;

    @BeforeEach
    public void setUp()
    {
        tom       = new Player("Tom", null);
        aCylon    = Player.REFERENCE_CYLON.clone();

        emptyBoard = new Board();

        aGame = Game.builder()
            .withPlayer(tom)
            .withPlayer(aCylon)
            .build();
    }

    @Test
    public void testConstructor()
    {
        assertThat(aGame.getPlayer1(), equalTo(tom));
        assertThat(aGame.getPlayer2(), equalTo(aCylon));

        assertThat(aGame.isOver(), is(false));

        assertThat(aGame.getWinner(), is(nullValue()));
        assertThat(aGame.getLoser(), is(nullValue()));

        // Can not test without Board.equals method
        assertThat(aGame.getBoard(), equalTo(emptyBoard));
    }

    @Test
    public void testPlayMatch()
    {
        fail("Not yet implemented");
    }

    @Test
    public void testPlayRound()
    {
        fail("Not yet implemented");
    }


    @Test
    public void testPerformTurn()
    {
        fail("Not yet implemented");
    }
}
