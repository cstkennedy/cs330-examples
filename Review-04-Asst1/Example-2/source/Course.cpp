#include <iomanip>

#include "Course.h"

/**
 *
 */
Course::Course()
    :number("ERROR 000"),
     crn(0),
     credits(0)
{
}

/**
 *
 */
Course::Course(std::string n, int c, int h)
    :number(n),
     crn(c),
     credits(h)
{
}

/**
 *
 */
void Course::display(std::ostream &outs) const{
    outs << credits << " credits - " 
         << number << " (CRN " << crn << ")";  
}