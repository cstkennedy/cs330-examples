#ifndef COURSE_H_INCLUDED
#define COURSE_H_INCLUDED

#include <iostream>
#include <string>

/**
 * Course represents an individual Course Section.
 * This is one offering of a class--e.g., a section
 * of CS 330 that meets from 0300pm to 0415pm on Tuesdays
 * and Thursdays.
 *
 * No two course sections share the same crn.
 */
class Course {
    private:
        /**
         * Course identifier--e.g., CS 330 or Math 211
         */
        std::string number;

        /**
         * 5 digit course reference number
         */
        int         crn;

        /**
         * Number of credit hours
         */
         int        credits;

    public:
        /**
         * Default to number = ERROR 000 and crn = 0 
         */
        Course();

        /**
         * Create an Course with a specified number and crn
         *
         * @param n course name
         * @param c course reference number (crn)
         * @param h credit hours
         */
        Course(std::string n, int c, int h);

        /**
         * Retrieve course number
         */
        std::string getNumber() const;

        /**
         * Change course number
         */
        void setNumber(std::string n);

        /**
         * Retrieve CRN
         */
        int getCrn() const;

        /**
         * Change CRN  
         *
         * @pre i is a 5 digit integer
         */
        void setCrn(int i);

        /**
         * Retrieve credit hours
         */
        int getCredits() const;

        /**
         * Check for logical equivalence--based on course number
         */
        bool operator==(const Course &rhs) const;

        /**
         * Check ordering--based on course number
         */
        bool operator<(const Course &rhs) const;

        /**
         * Print one Course
         */
        void display(std::ostream &outs) const;
};

/**
 * 
 */
inline int Course::getCrn() const
{
    return this->crn;
}

/**
 * 
 */
inline void Course::setCrn(int i)
{
    this->crn = i;
}

/**
 * 
 */
inline std::string Course::getNumber() const
{
    return this->number;
}

/**
 * 
 */
inline
void Course::setNumber(std::string n)
{
    this->number = n;
}

/**
 * 
 */
inline
int Course::getCredits() const
{
    return credits;
}

/**
 *
 */
inline
bool Course::operator==(const Course &rhs) const
{
    return this->number == rhs.number; 
}

/**
 *
 */
inline
bool Course::operator<(const Course &rhs) const
{
    return this->number < rhs.number;
}

/**
 * Stream Insertion Operator
 * <p>
 * Print one Course by invoking display
 */
inline
std::ostream& operator<<(std::ostream &outs, const Course &prt)
{
    prt.display( outs );

    return outs;
}

#endif