#include <iostream>
#include <iomanip>
#include <cstdlib>
#include <cmath>
#include <string>
#include <utility>

#include "Player.h"
#include "Board.h"
#include "Game.h"


using namespace std;

/**
 *
 */
int main()
{
    // Player test
    {
        Player tom("Tom");

        std::cout << tom << "\n";
    }

    // Board test
    {
        // output true/false instead of 1/0
        std::cout.setf(std::ios::boolalpha);

        Board board;

        std::cout << board << "\n";
        std::cout << board.isFull() << "\n";

        for (int i = 0 ; i < 9; i++) {
            board.setCell((i+1), 'X');
        }

        std::cout << board << "\n";
        std::cout << board.isFull() << "\n";
    }

    // Game test
    {
        Player tom("Thomas");
        Player jay("Jay");

        Game game(tom, jay);

        //while(!(game.getBoard().isFull())) {
        while(game.isNotOver()) {
            game.playRound();
        }
    }

    return 0;
}