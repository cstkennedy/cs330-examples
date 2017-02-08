#ifndef INVENTORY_H_INCLUDED
#define INVENTORY_H_INCLUDED

#include <iostream>

#include "Course.h"
#include "Schedule.h"

/**
 * A Student has a name and a schedule.
 * <p>
 * For simplicity, we will assume no two students
 * share the same name. We will forgo a more
 * formal student id.
 */
class Student{
    private:
       std::string name;
       Schedule    schedule;

    public:
        /**
         * Default to a student with
         * the name ERROR
         */
        Student();

        /**
         * Create a student with a specified name
         *
         * @param n desired name
         */
        Student(std::string n);

        /**
         * Retrieve the name attribute
         */
        std::string getName() const;

        /**
         * Attempt to enroll in a course.
         *
         * @return true if the course was successfully added to 
                the student schedule and false otherwise
         */
        bool enrollIn(const Course &toAdd);

        /**
         * Print a Summary of the Student and his/her schedule
         */
        void display(std::ostream &outs) const;

        /**
         * Logical Equivalence Operator
         */
        bool operator==(const Student& rhs) const;

        /**
         * Less-Than (Comes-Before) Operator
         */
        bool operator<(const Student& rhs) const;
};

/**
 *
 */
inline
std::string Student::getName() const
{
    return name;
}

/**
 *
 */
inline
bool Student::enrollIn(const Course &toAdd)
{
    return schedule.add(toAdd);
}

/**
 * Print the Student through use of the display member function
 */
inline 
std::ostream& operator<<(std::ostream &outs, const Student &prt)
{
    prt.display(outs);
    
    return outs;
}

#endif