package edu.odu.cs.cs330.examples.tictactoe;

import java.io.IOException;

/**
 * This is more a Player interface than a Player class.
 * <p>
 * However, such distinctions and discussions belong in
 * the OOP and Inheritance Modules
 */
class Player implements Cloneable
{
    /**
     * A Player that serves as a sentinal value or placeholder.
     */
    public static final Player REFERENCE_CYLON = new Player();

    /**
     * Checks whether a player is a placeholder or
     * an actual player.
     *
     * @param possibleCylon the player whose humanity is in question
     *
     * @return true if the player is a Cylon
     */
    public static boolean isGeneric(final Player possibleCylon)
    {
        return possibleCylon.equals(Player.REFERENCE_CYLON);
    }

    /**
     * Name of a player.
     */
    private String name;

    /**
     * Move selection strategy
     */
    private Strategy myStrategy;

    /**
     * Create a Player with a Generic name.
     */
    private Player()
    {
        this.name = "I. C. Generic";
        this.myStrategy = null;
    }

    /**
     * Create a Player with a selected name.
     *
     * @param n desired name
     * @param strategy selected strategy
     */
    public Player(String n, Strategy strategy)
    {
        this.name = n;
        this.myStrategy = strategy;
    }

    /**
     * Retrieve name.
     *
     * @return player name
     */
    public String getName()
    {
        return name;
    }

    /**
     * Set player name.
     *
     * @param n new name
     *
     * @pre (n.size() > 0)
     */
    public void setName(String n)
    {
        name  = n;
    }

    /**
     * Retrieve the next move.
     *
     * @return board cell id representing the selected move
     *
     * @throws IOException if the move can not be retreived from the player.
     */
    public int nextMove()
        throws IOException
    {
        return this.myStrategy.nextMove();
    }

    /**
     * Is this a Human Player?
     * <p>
     * In this discussion, always yes :(
     *
     * @return true if the player is a human
     */
    public boolean isHuman()
    {
        return true;
    }

    /**
     * Is this a Computer Player?
     * <p>
     * In this discussion, always no :(
     *
     * @return true if the player is a Cylon
     */
    public boolean isComputer()
    {
        return false;
    }

    @Override
    public boolean equals(Object rhs)
    {
        if (!(rhs instanceof Player)) {
            return false;
        }

        Player rhsP = (Player) rhs;

        return this.name.equals(rhsP.name);
    }

    @Override
    public int hashCode()
    {
        return this.name.hashCode();
    }

    /**
     * Generate a player string, but only the name.
     */
    @Override
    public String toString()
    {
        return this.getName();
    }

    @Override
    public Player clone()
    {
        return new Player(this.name, null);
    }
}
