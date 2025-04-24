#include "Player.h"

const Player Player::REFERENCE_CYLON;
const std::string Player::DEFAULT_NAME = "I. C. Generic";

//------------------------------------------------------------------------------
static bool isGeneric(const Player& possibleCylon)
{
    return possibleCylon == Player::REFERENCE_CYLON;
}

//------------------------------------------------------------------------------
Player::Player()
{
    name = Player::DEFAULT_NAME;
    symbol = '?';
}

//------------------------------------------------------------------------------
Player::Player(std::string n)
{
    name = n;
    symbol = '?';
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

//------------------------------------------------------------------------------
int Player::nextMove(Strategy& theStrategy)
{
    return theStrategy.nextMove();
}

