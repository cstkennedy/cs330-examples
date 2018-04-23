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
            
        };

        Board board;

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

        bool playRound();

        const Player& getPlayer1() const;

        const Player& getPlayer2() const;

        const Player& getWinner() const;

        const Player& getLoser() const;

        bool endedWithWin() const;

        bool endedWithStalemate() const;

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