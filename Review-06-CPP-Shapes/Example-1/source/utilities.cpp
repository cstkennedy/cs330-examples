//Programmer : Thomas Kennedy

#include "utilities.h"

namespace utilities{
    /**
     *
     */
    bool readLine( std::istream& in_stream, std::string &value_in)
    {
        getline( in_stream, value_in);

        //Return False on read error
        return in_stream.good();
    }

    /**
     *
     */
    bool readLine( std::istream &in_stream, std::string &value_in, std::string message, int width)
    {
        std::cout << std::left << std::setw( width ) << message << ": ";

        return readLine( in_stream, value_in );
    }

    /**
     *
     */
    void printHorizontalLine( std::ostream& outs, char line_char, int width )
    {
        outs << std::setfill( line_char ) << std::left << std::setw( width ) << line_char << "\n";
        //reset outs fill
        outs.fill( ' ' );
    }

    /**
     *
     */
    std::string generateNumberTitle( std::string title_text, int title_num )
    {
        std::stringstream magic;
        
        magic << title_text << " " << title_num;
        
        return (magic.str());     
    }

    /**
     *
     */
    void printCenteredTitle( std::ostream& outs, std::string title, int width )
    {
        int magic_width = 0;
        
        magic_width = (width/2) - (title.length()/2) + title.length();

        outs << std::right << std::setw( magic_width ) << title << "\n"
             << std::left;
    }

    /**
     *
     */
    void printProjectHeading( std::ostream& outs, const std::string titles[], int title_items, int width )
    {
        //Output the top line
        printHorizontalLine( outs, '*', width );
        
        //Output the title text
        for( int i = 0; i < title_items; i++ ){
             printCenteredTitle( outs, titles[ i ], width );
        }          
        //output the bottom line
        printHorizontalLine( outs, '*', width );
    }

    /**
     *
     */
    void printHeading( std::ostream& outs, std::string title, int width, char border_char )
    {
        printHorizontalLine( outs, border_char, width );
        printCenteredTitle( outs, title, width);
        printHorizontalLine( outs, border_char, width );
        
        //reset outs
        outs.clear();
        outs.fill(' ');
    }

    /**
     *
     */
    bool areEqual( double lhs, double rhs, double threshold )
    {
        return std::abs( rhs - lhs ) < threshold;
    }

}