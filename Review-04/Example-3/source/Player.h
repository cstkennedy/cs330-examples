#ifndef PLAYER_H_INCLUDED
#define PLAYER_H_INCLUDED

#include <string>
#include <iostream>

/**
 * This is more a Player interface than a Player class.
 * <p>
 * However, such distinctions and discussions belong in 
 * the OOP and Inheritance Modules
 */
class Player {
    private:
        /**
         * Message used to prompt a human player for a move
         */
        static std::string PROMPT_MSG;

        std::string name;
        char        symbol;

    public:
        /**
         * Create a Player with a Generic name
         */
        Player();

        /**
         * Create a Player with a selected name
         *
         * @param n desired name
         */
        Player(std::string n);

        /**
         * Retrieve name
         */
        std::string getName() const;

        /**
         * Set player name
         *
         * @param n new name
         *
         * @pre (n.size() > 0)
         */
        void setName(std::string n);

        /**
         * Retrieve the next move
         */
        int nextMove();

        /**
         * Is this a Human Player?
         * <p>
         * In this discussion, always yes :(
         */
        bool isHuman() const;

        /**
         * Is this a Computer Player?
         * <p>
         * In this discussion, always no :(
         */
        bool isComputer() const;

        /**
         * Retrieve player symbol to be used
         * for marking moves
         */
        char getSymbol() const;

        /**
         * Change the player symbol
         */
        char setSymbol(char newSymbol);
};

/**
 * 
 */
inline
std::string Player::getName() const
{
    return name;
}

/**
 * 
 */
inline
void Player::setName(std::string n)
{
    name  = n;
}

/**
 * 
 */
inline
char Player::getSymbol() const
{
    return symbol;
}

/**
 * 
 */
inline
char Player::setSymbol(char newSymbol)
{
    symbol = newSymbol;
}

/**
 * Compare two players based on name
 *
 * _I had to add this when writing the Game ADT_
 */
inline
bool operator==(const Player& lhs, const Player& rhs)
{
    return lhs.getName() == rhs.getName();
}

/**
 * Output a player, but only the name
 */
inline
std::ostream& operator<<(std::ostream& outs, const Player& prt)
{
    outs << prt.getName();

    return outs;
}

#endif