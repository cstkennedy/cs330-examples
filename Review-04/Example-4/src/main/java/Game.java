import java.io.IOException;

/**
 * Orchestrates a single match of Tic-Tac-Toe.
 */
public class Game
{
    /**
     * The Meta-player that checks game status
     * e.g., checks for wins, who won, if there is
     * a stalemate.
     *
     * It is an implementation detail that is not
     * exposed to the outside world;
     */
    private class Referee {
        /**
         * reference to the gameboard
         */
        private final Board boardRef;

        private Referee()
        {
            boardRef = null;
        }

        /**
         * Create the referee and allow access
         * to a board through a constant reference variable
         */
        public Referee(final Board b)
        {
            this.boardRef = b;
        }

        /**
         * Check for a win condition.
         *
         * @return 1 if player1, or 2 if player2 won.
         *     A 0 indicates neither won
         */
        public int checkForWin()
        {
            int winner = checkForHorizontalWin();

            if (winner != 0) {
                return winner;
            }

            winner = checkForVerticalWin();

            if (winner != 0) {
                return winner;
            }

            winner = checkForDiagonalWin();

            if (winner != 0) {
                return winner;
            }

            return 0;
        }

        /**
         * Determine whether a cell in the board has been selected
         * by a player.
         *
         * @param move player candidate move
         *
         * @return true if the cell is currently empty
         */
        public boolean selectedCellIsEmpty(int move)
        {
            return boardRef.getCell(move) != 'X' && boardRef.getCell(move) != 'O';
        }


        /**
         * Check each row of the board for three 'X' or three 'O'
         * characters.
         *
         * @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
         *     no one has won
         */
        public int checkForHorizontalWin()
        {
            Board.Pair[] triple = boardRef.get3Cells(1, 2, 3);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second);
            }

            triple = boardRef.get3Cells(4, 5, 6);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second);
            }

            triple = boardRef.get3Cells(7, 8, 9);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second);
            }

            return 0;
        }

        /**
         * Check each column of the board for three 'X' or three 'O'
         * characters.
         *
         * @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
         *     no one has won
         */
        public int checkForVerticalWin()
        {
            Board.Pair[] triple = boardRef.get3Cells(1, 4, 7);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second); // Oops mistake
            }

            triple = boardRef.get3Cells(2, 5, 8);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second);
            }

            triple = boardRef.get3Cells(3, 6, 9);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second);
            }

            return 0;
        }

        /**
         * Check the two diagonals of the board for three 'X' or three 'O'
         * characters.
         *
         * @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
         *     no one has won
         */
        public int checkForDiagonalWin()
        {
            Board.Pair[] triple = boardRef.get3Cells(1, 5, 9);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second);
            }

            triple = boardRef.get3Cells(7, 5, 3);

            if (allThreeMatch(triple)) {
                // if they match, grab the 'X' or 'O'
                return playerNumFromSymbol(triple[0].second);
            }

            return 0;
        }

        /**
         * Check for three matching symbols in the Pair-Triple.
         *
         * @return true if all three pairs contain the same symbol
         */
        public boolean allThreeMatch(Board.Pair[] triple)
        {
            char firstVal = triple[0].second; // std::pair first and second

            int numMatches = 0;

            for (int i = 0; i < triple.length; i++) {
                if (firstVal == triple[i].second) {
                    numMatches++;
                }
            }

            return (numMatches == 3);
        }

        /**
         * Given an 'X' or an 'O' determine which player is using the symbol.
         *
         * @return 1 for player 1 or 2 for player 2
         *
         * @pre sym == 'X' or 'O'
         */
        public int playerNumFromSymbol(char sym)
        {
            /*
            if (sym == 'X') {
                return 1;
            }
            return 2;
            */

            return (sym == 'X' ? 1 : 2);
        }
    }

    private Board board;
    private Referee ref; // The referee for this game

    private Player player1;
    private Player player2;

    private Player winner;

    private int numMovesMade;

    // Disable Game Default Constructor
    private Game()
    {

    }

    /**
     * Construct a Game setting player 1 and player 2.
     */
    public Game(Player p1, Player p2)
    {
        player1 = p1;
        player2 = p2;

        board = new Board();
        ref = new Referee(this.board);

        player1.setSymbol('X');
        player2.setSymbol('O');
    }

    /**
     * Play one round of Tic-Tac-Toe.
     *
     * @return true if the game ended during the round
     *
     * @throws IOException if there is an error reading the selected move
     */
    public boolean playRound()
        throws IOException
    {
        // The game ended already - assert could be used
        if (board.isFull()) {
            return true;
        }

        int winnerId = 0;

        System.out.println(board);
        roundTurn(player1);

        // The game is over
        if (board.isFull()) {
            System.out.println(board);
            return true;
        }

        winnerId = ref.checkForWin();

        if (winnerId == 1) {
            winner = player1;
            return true;
        }

        System.out.println(board);
        roundTurn(player2);

        // Final board
        System.out.println(board);

        winnerId = ref.checkForWin();

        if (winnerId == 2) {
            winner = player2;
            return true;
        }

        return false;
    }

    public Player getPlayer1()
    {
        return player1;
    }

    public Player getPlayer2()
    {
        return player2;
    }

    public Player getWinner()
    {
        return winner;
    }

    public Player getLoser()
    {
        // Stalemate
        if (endedWithStalemate()) {
            //return Player::referenceCylon;
            return null;
        }

        // There was a win, figure out who lost
        if (winner.equals(player1)) {
            return player2;
        }

        return player1;
    }

    public boolean endedWithWin()
    {
        return (winner != null);
    }

    public boolean endedWithStalemate()
    {
        return (board.isFull()) && (winner == null);
    }

    public boolean isOver()
    {
        return (endedWithWin() || endedWithStalemate());
    }

    public boolean isNotOver()
    {
        return !isOver();
    }

    public Board getBoard()
    {
        return board;
    }

    /**
     * Get a player move, and update the board.
     */
    private boolean roundTurn(Player player)
        throws IOException
    {
        int   move;
        char  sym;

        move = player.nextMove();
        sym  = player.getSymbol();

        //while (board.getCell(move) != 'X' && board.getCell(move) != 'O') {
        while (!ref.selectedCellIsEmpty(move)) {
            move = player.nextMove();
            sym  = player.getSymbol();
        }
        board.setCell(move, sym);

        // @todo add move validation
        return true;
    }

};
