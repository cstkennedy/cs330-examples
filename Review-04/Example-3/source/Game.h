#ifndef GAME_H_INCLUDED
#define GAME_H_INCLUDED

#include "Player.h"
#include "Board.h"

class Game {
    private:
        /**
         * The Meta-player that checks gaome status
         * e.g., checks for wins, who won, if there is 
         * a stalemate.
         *
         * It is an implementation detail that is not
         * exposed to the outside world;
         */
        class Referee {
            const Board& boardRef; // reference to the gameboard

            public:
                Referee() = delete;

                /**
                 * Create the referee and allow access
                 * to a board through a constant reference variable
                 */
                Referee(const Board& b);

                /**
                 * Check for a win condition
                 *
                 * @return 1 if player1, or 2 if player2 won.
                 *     A 0 indicates neither won
                 */
                int checkForWin() const;

            private:

                int checkForHorizontalWin() const;

                int checkForVerticalWin() const;

                int checkForDiagonalWin() const;

                bool allThreeMatch(const Board::CellTriple& triple) const;

                /**
                 * @pre sym == 'X' or 'O'
                 */
                int playerNumFromSymbol(char sym) const;
        };

        Board board;
        Referee ref; // The referee for this game

        Player& player1;
        Player& player2;

        Player* winner;

        int numMovesMade;

    public:
        // Disable Game Default Constructor
        Game() = delete;

        /**
         * Construct a Game setting player 1 and player 2
         */
        Game(Player& p1, Player& p2);

        /**
         * Play one round of Tic-Tac-Toe
         *
         * @return true if the game ended during the round
         *
         */
        bool playRound();

        const Player& getPlayer1() const;

        const Player& getPlayer2() const;

        const Player& getWinner() const;

        const Player& getLoser() const;

        bool endedWithWin() const;

        bool endedWithStalemate() const;

        bool isOver() const;

        bool isNotOver() const;

        const Board& getBoard() const;

    private:
        /**
         * Get a player move, and update the board
         */
        bool roundTurn(Player& player);

};

/**
 *
 */
inline
const Player& Game::getPlayer1() const
{
    return player1;
}

/**
 *
 */
inline
const Player& Game::getPlayer2() const
{
    return player2;
}

/**
 *
 */
inline
const Player& Game::getWinner() const
{
    return (*winner);
}


/**
 *
 */
inline
const Board& Game::getBoard() const
{
    return board;
}

#endif