#include <utility>
#include "Student.h"

// Allow the compiler to define the remaining 
// comparison operators
using namespace std::rel_ops;

/**
 *
 */
Student::Student()
    :name("Error")
{
}

/**
 *
 */
Student::Student(std::string n)
    :name(n)
{
}

/**
 *
 */
void Student::display(std::ostream &outs) const
{
    outs << " " << name << "\n";
    outs << schedule;
}

/**
 *
 */
bool Student::operator==(const Student& rhs) const
{
    return this->name == rhs.name;
}

/**
 *
 */
bool Student::operator<(const Student& rhs) const
{
    return this->name < rhs.name;
}