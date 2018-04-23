#include "Game.h"

/**
 *
 */
Game::Referee::Referee(const Board& b)
    :boardRef(b)
{
    // Nothing else
}

/**
 *
 */
int Game::Referee::checkForWin() const
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
 *
 */
int Game::Referee::checkForHorizontalWin() const
{
    Board::CellTriple triple = boardRef.get3Cells(1, 2, 3);

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
 *
 */
int Game::Referee::checkForVerticalWin() const
{
    Board::CellTriple triple = boardRef.get3Cells(1, 4, 7);

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
 *
 */
int Game::Referee::checkForDiagonalWin() const
{
    Board::CellTriple triple = boardRef.get3Cells(1, 5, 9);

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
 *
 */
bool Game::Referee::allThreeMatch(const Board::CellTriple& triple) const
{
    Board::CellValue firstVal = triple[0].second; // std::pair first and second

    int numMatches = 0;

    for(int i = 0; i < triple.size(); i++) {
        if (firstVal == triple[i].second) {
            numMatches++;
        }
    }

    return (numMatches == 3);
}

/**
 *
 */
int Game::Referee::playerNumFromSymbol(char sym) const
{
    /*
    if (sym == 'X') {
        return 1;
    }
    return 2;
    */

    return (sym == 'X' ? 1 : 2);
}

//------------------------------------------------------------

/**
 *
 */
Game::Game(Player& p1, Player& p2)
    :player1(p1),
     player2(p2),
     winner(nullptr),
     ref(this->board)
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

    int winnerId = 0;

    std::cout << board << "\n";
    roundTurn(player1);

    // The game is over
    if(board.isFull()) {
        std::cout << board << "\n";
        return true;
    }

    winnerId = ref.checkForWin();

    if (winnerId == 1) {
        winner = &player1;
        return true;
    }

    std::cout << board << "\n";
    roundTurn(player2);

    // Final board
    std::cout << board << "\n";

    winnerId = ref.checkForWin();
    
    if (winnerId == 2) {
        winner = &player2;
        return true;
    }

    return false;
}

/**
 *
 */
bool Game::endedWithWin() const
{
    return (winner != nullptr);
}

/**
 *
 */
bool Game::endedWithStalemate() const
{
    return (board.isFull()) && (winner == nullptr);
}

bool Game::isOver() const
{
    return (endedWithWin() || endedWithStalemate());
}

bool Game::isNotOver() const
{
    return !isOver();
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