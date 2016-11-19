//Programmer : Thomas Kennedy

package utilities;

import java.lang.StringBuilder;

public final class Utilities{
    public static final int    W_WIDTH = 80;   ///< Width of the terminal window    
    public static final double EPS     = 1E-6; ///< Default Precision (epsilon)

    /**
     * Print a horizontal line
     */
    public static String horizontalLine(char line_char, int width){
        return String.format("%0" + width + "d", 0).replace("0", "" + line_char);
    }

    /**
     *  Print a centered title
     */
    public static String centeredTitle(String title, int width)
    {
        int magic_width = 0;
        
        magic_width = (width/2) - (title.length()/2) + title.length();

        return(
            String.format("%" + magic_width + "s", title) + 
            "\n"
        );
    }

    /**
     *  Print a heading followed by a horizontal rule
     * 
     *  @param heading the title to display
     *  @param width the width of the heading
     *  @param border the character with which to create the horizontal rule
     */
    public static String seperatedHeading(String heading, int width, char border)
    {
        return(
            centeredTitle(heading, width) +
            horizontalLine(border, width)
        );
    } 

    /**
     *  Print the stylized program/project heading
     */
    public static String projectHeading(String[] titles, int width)
    {
        StringBuilder bld = new StringBuilder();

        bld.append(horizontalLine('*', width));
        bld.append("\n");

        for(String line : titles){
            bld.append(centeredTitle(line, width));
        }

        bld.append(horizontalLine('*', width));

        return bld.toString();
    }

    /**
     *  Print a centered heading
     */
    public static String heading(String title, int width, char border)
    {
        return(            
            horizontalLine(border, width) + "\n" +
            centeredTitle(title, width  ) +
            horizontalLine(border, width)
       );
    }

    /**
     *  Compare two floating point numbers - determine if equal within
     *  a specified threshold
     */
    public static boolean areEqual(double lhs, double rhs, double threshold)
    {
        return (
            Math.abs(rhs - lhs) < threshold
       );
    }
}