// Programmer : Thomas Kennedy

#include "utilities.h"

namespace utilities {
    //--------------------------------------------------------------------------
    bool readLine(std::istream& in_stream, std::string &value_in)
    {
        getline(in_stream, value_in);

        // Return False on read error
        return in_stream.good();
    }

    //--------------------------------------------------------------------------
    bool readLine(std::istream &in_stream, std::string &value_in,
                  std::string message, int width)
    {
        std::cout << std::left << std::setw(width) << message << ": ";

        return readLine(in_stream, value_in);
    }

    //--------------------------------------------------------------------------
    std::string generateNumberTitle(std::string_view title_text, int title_num)
    {
        std::stringstream magic;

        magic << title_text << " " << title_num;

        return (magic.str());
    }

    //--------------------------------------------------------------------------
    void printCenteredTitle(std::ostream& outs, std::string_view title, int width)
    {
        int magic_width = 0;

        //magic_width = (width / 2) - (title.length() / 2) + title.length();
        magic_width = (width >> 1) - (title.length() >> 1) + title.length();

        outs << std::right << std::setw(magic_width) << title << "\n"
             << std::left;
    }

    //--------------------------------------------------------------------------
    bool areEqual(double lhs, double rhs, double threshold)
    {
        return std::abs(rhs - lhs) < threshold;
    }

}
