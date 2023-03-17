#ifndef STRATEGY_H_DEFINED
#define STRATEGY_H_DEFINED

#include <string>

class Strategy
{
    public:
        virtual int nextMove() = 0;
};

class KeyboardStrategy: public Strategy
{
    private:
        std::string _name;

    public:
        static const std::string PROMPT_MSG;

        KeyboardStrategy(std::string name);

        ~KeyboardStrategy();

        int nextMove() override;
};

#endif
