#! /usr/bin/env python                          

# Thomas Kennedy
# February 2014
# Assignment 3 Solution

import os
import sys
import random
import math

def printHeading(title, outs=sys.stdout):
    print("=" * 40, file=outs)
    print(title.center(40), file=outs)
    print("=" * 40, file=outs)

#Main Function
def main():
    
    file_name = str( input("Enter input filename: ") )

    try:
        #open the output file
        out_file = open( "A3-cslogin.txt", "w")

        #open the input file
        in_file = open( file_name, "r")
    except:
        print("\nError: Could not open file.\n")
        sys.exit(1)

    length, width, radius = [float(x) for x in in_file.read().split()]

    #Perform Calculations
    rectangle_perimeter = 2 * ( length + width )
    rectangle_area      = length * width

    circle_perimeter    = 2 * math.pi * radius
    circle_area         = math.pi * ( radius ** 2.0 )
          
    #Output to the screen and file
    for output_sd in [sys.stdout, out_file]:
        printHeading( "Summary", output_sd )

        print("Rectangle:", file=output_sd) 
        print("  {:<16}: {:>10.2f}".format( "Length",    length    ), file=output_sd)    
        print("  {:<16}: {:>10.2f}".format( "Width" ,    width     ), file=output_sd)  
        print("  {:<16}: {:>10.2f}".format( "Perimeter", rectangle_perimeter  ), file=output_sd)  
        print("  {:<16}: {:>10.2f}".format( "Area",      rectangle_area ), file=output_sd)

        print("\nCircle:", file=output_sd) 
        print("  {:<16}: {:>10.2f}".format( "Radius", radius    ), file=output_sd)
        print("  {:<16}: {:>10.2f}".format( "Perimeter"  , circle_perimeter ), file=output_sd) 
        print("  {:<16}: {:>10.2f}".format( "Area", circle_area), file=output_sd)

        print("\nPerimeter:", file=output_sd)

        if rectangle_perimeter == circle_perimeter:
            print("  Rectangle Perimeter = Circle Perimeter", file=output_sd)
        elif rectangle_perimeter < circle_perimeter:
            print("  Rectangle Perimeter < Circle Perimeter", file=output_sd)
        else:
            print("  Rectangle Perimeter > Circle Perimeter", file=output_sd)

        print("\nArea:", file=output_sd)

        if rectangle_area == circle_area:
            print("  Rectangle Area = Circle Area", file=output_sd)
        elif rectangle_area < circle_area:
            print("  Rectangle Area < Circle Area", file=output_sd)
        else:
            print("  Rectangle Area > Circle Area", file=output_sd)        

    out_file.close()
    in_file.close()

if __name__ == "__main__":
     main()