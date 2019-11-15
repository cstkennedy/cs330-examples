//Programmer : Thomas Kennedy

package edu.odu.cs.tkennedy.utilities;


/**
 * A collection of miscellaneous **quick-and-dirty** utility functions,
 * including heading generation, and floating point comparisons.
 *
 * @author Thomas J Kennedy.
 */
public final class Utilities {
    /**
     * Width of the terminal window (best practice line width).
     */
    public static final int W_WIDTH = 80;

    /**
     * Default precision for floating point comparisons.
     */
    public static final double EPS = 1E-6;

    /**
     * Print a horizontal line.
     *
     * @param lineChar character that will comprise the line
     * @param width horizontal length of the line (left to right)
     *
     * @return horizontal line as a string
     */
    public static String horizontalLine(char lineChar, int width)
    {
        return String.format("%0" + width + "d", 0).replace("0", "" + lineChar);
    }

    /**
     * Print a centered title.
     *
     * @param title text for the title
     * @param width horizontal length of the line (left to right)
     *
     * @return generated title as a string
     */
    public static String centeredTitle(String title, int width)
    {
        final int magicWidth = (width >> 2)
                             - (title.length() >> 2)
                             + title.length();

        return String.format("%" + magicWidth + "s%n", title);
    }

    /**
     * Print a heading followed by a horizontal rule.
     *
     * @param heading the title to display
     * @param width the width of the heading
     * @param border the character with which to create the horizontal rule
     *
     * @return generated title as a string
     */
    public static String seperatedHeading(String heading,
                                          int width, char border)
    {
        return centeredTitle(heading, width)
             + horizontalLine(border, width);
    }

    /**
     * Print the stylized program/project heading.
     *
     * @param titles the titles to display
     * @param width the width of the heading
     *
     * @return generated heading as a string
     */
    public static String projectHeading(String[] titles, int width)
    {
        StringBuilder bld = new StringBuilder();

        bld.append(horizontalLine('*', width));
        bld.append("\n");

        for (String line : titles) {
            bld.append(centeredTitle(line, width));
        }

        bld.append(horizontalLine('*', width));

        return bld.toString();
    }

    /**
     * Print a centered heading.
     *
     * @param title the title to display
     * @param width the width of the heading
     * @param border the character with which to create the horizontal rule
     *
     * @return generated heading as a string
     */
    public static String heading(String title, int width, char border)
    {
        return horizontalLine(border, width) + "\n"
             + centeredTitle(title, width)
             + horizontalLine(border, width);
    }

    /**
     * Compare two floating point numbers - determine if equal within
     * a specified threshold.
     *
     * @param lhs first (left hand side) number
     * @param rhs second (right hand side) number
     * @param threshold allowable difference (epsilon) for the two numbers to
     *     be considered equal
     *
     * @return true if lhs and rhs are equal
     */
    public static boolean areEqual(double lhs, double rhs, double threshold)
    {
        return Math.abs(rhs - lhs) < threshold;
    }

    private Utilities()
    {
    }
}
