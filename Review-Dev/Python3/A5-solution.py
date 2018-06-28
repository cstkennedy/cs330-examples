#! /usr/bin/env python                          

# Thomas Kennedy
# February 2014
# Assignment 5 Solution

import os
import sys
import random
import math

def printHeading(title, outs=sys.stdout):
    print("=" * 40, file=outs)
    print(title.center(40), file=outs)
    print("=" * 40, file=outs)

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

    print("Rectangle:", file=outs) 
    print("  {:<16}: {:>10.2f}".format( "Length",    l    ), file=outs)    
    print("  {:<16}: {:>10.2f}\n".format( "Width" ,    w    ), file=outs)  
    print("  {:<16}: {:>10.2f}".format( "Perimeter", rect_perim  ), file=outs)  
    print("  {:<16}: {:>10.2f}".format( "Area",      rect_area ), file=outs)

    print("\nTriangle:", file=outs) 
    print("  {:<16}: {:>10.2f}".format( "Side A", a    ), file=outs)
    print("  {:<16}: {:>10.2f}".format( "Side B", b   ), file=outs)
    print("  {:<16}: {:>10.2f}\n".format( "Side C", c    ), file=outs)
    print("  {:<16}: {:>10.2f}".format( "Perimeter"  , triangle_perimeter ), file=outs) 
    print("  {:<16}: {:>10.2f}".format( "Area", triangle_area), file=outs)

    print("\nPerimeter:", file=outs)

    if rect_perim == triangle_perimeter:
        print("  Rectangle Perimeter = Triangle Perimeter", file=outs)
    elif rect_perim < triangle_perimeter:
        print("  Rectangle Perimeter < Triangle Perimeter", file=outs)
    else:
        print("  Rectangle Perimeter > Triangle Perimeter", file=outs)

    print("\nArea:", file=outs)

    if rect_area == triangle_area:
        print("  Rectangle Area = Triangle Area", file=outs)
    elif rect_area < triangle_area:
        print("  Rectangle Area < Triangle Area", file=outs)
    else:
        print("  Rectangle Area > Triangle Area", file=outs)        

#Main Function
def main():
    
    file_name = str( input("Enter input filename: ") )

    try:
        #open the output file
        out_file = open( "A5-cslogin.txt", "w")

        #open the input file
        in_file = open( file_name, "r")
    except:
        print("\nError: Could not open file.\n")
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