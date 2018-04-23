#include "Game.h"

/**
 *
 */
Game::Game(Player& p1, Player& p2)
    :player1(p1),
     player2(p2)
{
    player1.setSymbol('X');
    player2.setSymbol('O');
}

/**
 *
 */
bool Game::playRound()
{
    // The game ended already - assert could be used
    if(board.isFull()){
        return true;
    }

    /*
    int               move;
    Board::CellValue  sym;

    move = player1.nextMove();
    sym = player1.getSymbol();

    board.setCell(move, sym);

    // The game is over
    if(board.isFull()) {
        return true;
    }

    move = player2.nextMove();
    sym = player2.getSymbol();

    board.setCell(move, sym);
    */

    std::cout << board << "\n";
    roundTurn(player1);

    // The game is over
    if(board.isFull()) {
        std::cout << board << "\n";
        return true;
    }

    std::cout << board << "\n";
    roundTurn(player2);

    // Final board
    std::cout << board << "\n";

    return false;
}

/**
 *
 */
bool Game::endedWithWin() const
{

}

/**
 *
 */
bool Game::endedWithStalemate() const
{

}

/**
 *
 */
const Player& Game::getLoser() const
{
    // Stalemate
    if (endedWithStalemate()) {
        return Player();
    }

    // There was a win, figure out who lost
    if (*winner == player1) {
        return player2;
    }

    return player1;
}


/**
 * 
 */
bool Game::roundTurn(Player& player)
{
    int               move;
    Board::CellValue  sym;

    move = player.nextMove();
    sym = player.getSymbol();

    board.setCell(move, sym);

    // @todo add move validation
    return true;
}