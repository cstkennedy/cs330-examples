#ifndef SCHEDULE_H_INCLUDED
#define SCHEDULE_H_INCLUDED

#include <iostream>

#include "Course.h"

#define CREDIT_LIMIT 12

/**
 *
 */
class Schedule {
    private:
        /**
         * The atom of a Linked List
         */
        struct Node {
            Course data;
            Node*  next;

            /**
             * Construct a node that contains
             * a given room, with next set to null
             */
            Node(Course c)
                :data(c),
                 next(nullptr)
            {                
            }
        };

        /**
         * Beginning of the linked list
         */
        Node* head;

        /**
         * End of the linked list
         */
        Node* tail;

        /**
         * Total number of credits
         */
        int totalCredits;

    public:
        /**
         * Construct an empty schedule
         */
        Schedule();

        /**
         * Copy Constructor
         */
        Schedule(const Schedule& src);

        /**
         * Destructor
         */
        ~Schedule();

        /**
         * Retrieve the total number of credits
         */
        int getCredits() const;

        /**
         * Attempt to add a course
         */
        bool add(Course course);

        /**
         * Display a listing of each course and
         * the total number of credit hours
         */
        void display(std::ostream& outs) const;

        /**
         * Assignment Operator
         */
        Schedule& operator=(const Schedule& rhs);

    private:
        /**
         * Deconstruct a linked list
         */
        void deAllocateList();

        /**
         * Copy an existing Linked List
         */
        void copyList(const Schedule& rhs);
};

/**
 *
 */
inline
int Schedule::getCredits() const
{
    return totalCredits;
}

/**
 * Print the Schedule through use of the display member function
 */
inline 
std::ostream& operator<<(std::ostream &outs, const Schedule &prt)
{
    prt.display( outs );
    
    return outs;
}
#endif