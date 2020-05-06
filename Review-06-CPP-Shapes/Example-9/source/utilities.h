/** @file */

#ifndef Utilities_H_INCLUDED
#define Utilities_H_INCLUDED

#include <iostream>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <string>
#include <string_view>
#include <cstdlib>
#include <cmath>
#include <array>

/**
 * A collection of useful utilities.
 */
namespace utilities {
    constexpr int W_WIDTH = 70;  ///< Width of the terminal window

    // TJK - Removed input utility functions
    // TJK - Removed Number string title utility function
    // TJK - Added string_view concatenation helper

    inline
    std::string operator+(std::string_view lhs, std::string_view rhs)
    {
        return std::string(lhs.data()) + std::string(rhs.data());
    }

    /**
     * Print a horizontal line
     *
     * ostream is not constexpr
     */
    template<char line_char, int width = W_WIDTH>
    constexpr std::string generateHorizontalLine()
    {
        return std::string(width, line_char) + "\n";
    }

    /**
     * Print a centered title
     */
    template<int width = W_WIDTH>
    constexpr std::string generateCenteredTitle(std::string_view title)
    {
        const int magic_width = (width >> 1) - (title.length() >> 1);

        return std::string(magic_width, ' ') + title + "\n";
    }

    /**
     * Print a heading followed by a horizonal rule
     *
     * @param outs the output stream
     * @param heading the title to display
     * @param width the width of the heading
     * @param border the character with which to create the horizontal rule
     */
    template<char border_char = '-', int width = W_WIDTH>
    constexpr std::string generateSeperatedHeading(std::string_view heading)
    {
        return generateCenteredTitle<width>(heading)
             + generateHorizontalLine<border_char, width>();
    }

    /**
     * Print the stylized program/project heading
     *
     * @param outs output stream
     * @param titles[] list of title lines
     * @param title_items number of title lines
     * @param width heading width
     */
    template <size_t NUM_LINES, int width = W_WIDTH>
    std::string generateProjectHeading(const std::array<std::string_view, NUM_LINES>& titles)
    {
        std::string the_heading;

        // Output the top line
        the_heading += generateHorizontalLine<'*', width>();

        // Output the title text
        for (std::string_view line : titles) {
             the_heading += generateCenteredTitle<width>(line);
        }
        // Output the bottom line
        the_heading += generateHorizontalLine<'*', width>();

        return the_heading;
    }

    /**
     * Print a centered heading
     * @param outs output stream
     * @param title heading
     * @param width heading width
     * @param border_char character out of whcih the dividing line is composed
     */
    template<char border_char, int width = W_WIDTH>
    constexpr std::string generateHeading(std::string_view title)
    {
        return generateHorizontalLine<border_char, width>()
             + generateCenteredTitle<width>(title)
             + generateHorizontalLine<border_char, width>();
    }

    /**
     * Print a blankline
     */
    inline
    void println(std::ostream& outs = std::cout)
    {
        outs << "\n";
    }

    /**
     * Trim leading and trailing whitespace from a string.
     *
     * @param str string to prune
     *
     * @pre str is nonempty
     */
    inline
    void trim(std::string &str)
    {
        // If the string is empty, do nothing
        if (str.empty()) {
            return;
        }

        const int first_nonspace = str.find_first_not_of(" \t");
        const int last_non_space = str.find_last_not_of(" \t");

        str = str.substr(first_nonspace,
                         last_non_space + 1);
    }

	/**
     * Compare two floating point numbers - determine if equal within
     * a specified threshold
     *
     * @param lhs first number
     * @param rhs second number
     * @param threshold tolerance
     */
    bool areEqual(double lhs, double rhs, double threshold = 1e-6);
}
#endif
