//Programmer : Thomas Kennedy

/** @file */

#ifndef Utilities_H_INCLUDED
#define Utilities_H_INCLUDED

#include <iostream>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <string>
#include <cstdlib>
#include <cmath>

namespace utilities{
    const int W_WIDTH = 70; ///< Width of the terminal window

    ///
    /// Utility Function to read a value from an input stream ( any type with a operator>> defined ) - non-interactive
    ///
    template<class T>
    bool readValue( std::istream &in_stream, T &value_in){
        in_stream >> value_in;
        
        //Return False on read error
        return in_stream.good();
    }

    ///
    /// Utility Function to read a value from an input stream - interactive
    ///
    template<class T>
    bool readValue( std::istream &in_stream, T &value_in, std::string message, int width=34 ){
        std::cout << std::left << std::setw( width ) << message << ": ";

        return readValue( in_stream, value_in );
    }

    ///
    /// Utility function to read a line - non-interactive
    ///
    bool readLine( std::istream &in_stream, std::string &value_in);

    ///
    /// Utility function to read a line - interactive
    ///
    bool readLine( std::istream &in_stream, std::string &value_in, std::string message, int width=34 );

    ///
    /// Print a horizontal line
    ///
    void printHorizontalLine( std::ostream& outs, char line_char, int width );

    ///
    /// Combine a title string and number into one string
    ///
    std::string generateNumberTitle( std::string title_text, int title_num );

    ///
    /// Print a centered title
    ///
    void printCenteredTitle( std::ostream& outs, std::string title, int width );

    ///
    /// Print a heading followed by a horizonal rule
    ///
    /// \param outs the output stream
    /// \param heading the title to display
    /// \param width the width of the heading
    /// \param border the character with which to create the horizontal rule
    ///
    inline void printSeperatedHeading(
        std::ostream &outs, std::string heading,
        int width, char border='-'
    ){
        printCenteredTitle(  outs, heading, width );
        printHorizontalLine( outs, border, width );
    } 

    ///
    /// Print the stylized program/project heading
    ///
    void printProjectHeading( std::ostream& outs, const std::string titles[], int title_items, int width=W_WIDTH );

    ///
    /// Print a centered heading
    ///
    void printHeading( std::ostream& outs, std::string title, int width=W_WIDTH, char border_char = '*' );

    ///
    /// Compare two floating point numbers - determine if equal within
    /// a specified threshold
    ///
    bool areEqual( double lhs, double rhs, double threshold=1e-6 );

}
#endif