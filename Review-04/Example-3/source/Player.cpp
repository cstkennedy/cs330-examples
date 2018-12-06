#include "Player.h"

std::string Player::PROMPT_MSG = "Enter your desired move (1-9): ";

const Player Player::referenceCylon;

//------------------------------------------------------------------------------
static bool isGeneric(const Player& possibleCylon)
{
    return possibleCylon == Player::referenceCylon;
}


//------------------------------------------------------------------------------
Player::Player()
{
    name = "I. C. Generic";
}

//------------------------------------------------------------------------------
Player::Player(std::string n)
{
    name = n;
}

//------------------------------------------------------------------------------
int Player::nextMove()
{
    int choice;

    std::cout << (name + ", " + Player::PROMPT_MSG);
    std::cin >> choice;

    return choice;
}

//------------------------------------------------------------------------------
bool Player::isHuman() const
{
    return true;
}

//------------------------------------------------------------------------------
bool Player::isComputer() const
{
    return false;
}

