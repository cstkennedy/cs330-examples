#ifndef GAME_H_INCLUDED
#define GAME_H_INCLUDED

class Game {
    private:
        Board board;

        Player player1;
        Player player2;

        int numMovesMade;

        class Referee {
            
        }

    public:

        bool playRound();

        const Player& getWinner() const;

        const Player& getLoser() const;

        bool endedWithWin() const;

        bool endedWithStalemate() const;

        const Board& getBoard() const;

}

#endif