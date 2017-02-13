#ifndef COLUMN_HEADING_H_INCLUDED
#define COLUMN_HEADING_H_INCLUDED

#include <iostream>
#include <iomanip>
#include <string>

/**
 * Single Column Heading
 */
struct ColumnHeading {
    /**
     * Flag to indicate alignment direction
     */
    enum Alignment {
        LEFT,
        RIGHT
    };

    int          width;
    std::string  title;
    Alignment    alignment;

    /**
     * Non-Default Constructor
     */
    ColumnHeading(int w, std::string t, Alignment f);
};

inline
std::ostream& operator<<(std::ostream& outs, const ColumnHeading& prt)
{
    outs << (prt.alignment == ColumnHeading::Alignment::LEFT ? std::left : std::right) 
         << std::setw(prt.width) 
         << prt.title;

    return outs;
}


#endif