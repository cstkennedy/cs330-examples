#! /usr/bin/env python                          

# Thomas Kennedy
# February 2014
# Assignment 5 Solution

import os
import sys
import random
import math

def printHeading(title, outs=sys.stdout):
    print >> outs, "=" * 40
    print >> outs,title.center(40)
    print >> outs,"=" * 40

#
#
#
def computeTrianglePerimeter( a, b, c ):
    return a + b + c
#
#
#
def computeTriangleArea( a, b, c, perimeter ):
    s = (1.0/2) * perimeter;

    return math.sqrt( s * (s-a) * (s-b) * (s-c) )

#
#
#
def computeRectanglePerimeter( l, w ):
    return 2 * ( l + w )

#
#
#
def computeRectangleArea( l, w ):
    return l * w
#
#
#
def printFinalSummary( outs, a, b, c, triangle_perimeter,
                    triangle_area, l, w, rect_perim, rect_area):
    printHeading( "Summary", outs )

    print >> outs, "Rectangle:" 
    print >> outs, "  {:<16}: {:>10.2f}".format( "Length",    l    )    
    print >> outs, "  {:<16}: {:>10.2f}\n".format( "Width" ,    w    )  
    print >> outs, "  {:<16}: {:>10.2f}".format( "Perimeter", rect_perim  )  
    print >> outs, "  {:<16}: {:>10.2f}".format( "Area",      rect_area )

    print >> outs, "\nTriangle:" 
    print >> outs, "  {:<16}: {:>10.2f}".format( "Side A", a    )
    print >> outs, "  {:<16}: {:>10.2f}".format( "Side B", b   )
    print >> outs, "  {:<16}: {:>10.2f}\n".format( "Side C", c    )
    print >> outs, "  {:<16}: {:>10.2f}".format( "Perimeter"  , triangle_perimeter ) 
    print >> outs, "  {:<16}: {:>10.2f}".format( "Area", triangle_area)

    print >> outs, "\nPerimeter:"

    if rect_perim == triangle_perimeter:
        print >> outs, "  Rectangle Perimeter = Triangle Perimeter"
    elif rect_perim < triangle_perimeter:
        print >> outs, "  Rectangle Perimeter < Triangle Perimeter"
    else:
        print >> outs, "  Rectangle Perimeter > Triangle Perimeter"

    print >> outs, "\nArea:"

    if rect_area == triangle_area:
        print >> outs, "  Rectangle Area = Triangle Area"
    elif rect_area < triangle_area:
        print >> outs, "  Rectangle Area < Triangle Area"
    else:
        print >> outs, "  Rectangle Area > Triangle Area"        

#Main Function
def main():
    
    file_name = str( raw_input("Enter input filename: ") )

    try:
        #open the output file
        out_file = open( "A5-cslogin.txt", "w")

        #open the input file
        in_file = open( file_name, "r")
    except:
        print "\nError: Could not open file.\n"
        sys.exit(1)

    length, width, side_a, side_b, side_c = [float(x) for x in in_file.read().split()]

    #Perform Calculations
    rectangle_perimeter = computeRectanglePerimeter( length, width )
    rectangle_area      = computeRectangleArea( length, width )

    triangle_perimeter  = computeTrianglePerimeter( side_a, side_b, side_c )
    triangle_area       = computeTriangleArea( side_a, side_b, side_c, triangle_perimeter )
          
    #Output to the screen and file
    for outs in [sys.stdout, out_file]:
        printFinalSummary( outs, side_a, side_b, side_c, triangle_perimeter, triangle_area,
                       length, width, rectangle_perimeter, rectangle_area);

    out_file.close()
    in_file.close()

if __name__ == "__main__":
     main()