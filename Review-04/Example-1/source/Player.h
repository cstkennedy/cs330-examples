#ifndef PLAYER_H_INCLUDED
#define PLAYER_H_INCLUDED

/**
 * This is more a PLayer interface than a Player class.
 * <p>
 * However, such distinctions and discussions belong in 
 * the OOP and Inheritance Modules
 */
class Player {
    private:

    public:
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

        char getSymbol() const;

        char setSymbol(char newSymbol);
}

#endif