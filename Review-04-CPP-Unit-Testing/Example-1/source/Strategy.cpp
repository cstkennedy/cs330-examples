#include <iostream>
#include <string>

#include "Strategy.h"

const std::string KeyboardStrategy::PROMPT_MSG = "Enter your desired move (1-9): ";

KeyboardStrategy::KeyboardStrategy(std::string name)
    :_name(name)
{
}

KeyboardStrategy::~KeyboardStrategy()
{
}

int KeyboardStrategy::nextMove()
{
    int choice;

    std::cout << (this->_name + ", " + KeyboardStrategy::PROMPT_MSG);
    std::cin >> choice;

    return choice;
}
