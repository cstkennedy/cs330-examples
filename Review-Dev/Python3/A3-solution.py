#! /usr/bin/env python                          

# Thomas Kennedy
# February 2014
# Assignment 3 Solution

import os
import sys
import random
import math

def printHeading(title, outs=sys.stdout):
    print >> outs, "=" * 40
    print >> outs,title.center(40)
    print >> outs,"=" * 40

#Main Function
def main():
    
    file_name = str( raw_input("Enter input filename: ") )

    try:
        #open the output file
        out_file = open( "A3-cslogin.txt", "w")

        #open the input file
        in_file = open( file_name, "r")
    except:
        print "\nError: Could not open file.\n"
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

        print >> output_sd, "Rectangle:" 
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Length",    length    )    
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Width" ,    width     )  
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Perimeter", rectangle_perimeter  )  
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Area",      rectangle_area )

        print >> output_sd, "\nCircle:" 
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Radius", radius    )
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Perimeter"  , circle_perimeter ) 
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Area", circle_area)

        print >> output_sd, "\nPerimeter:"

        if rectangle_perimeter == circle_perimeter:
            print >> output_sd, "  Rectangle Perimeter = Circle Perimeter"
        elif rectangle_perimeter < circle_perimeter:
            print >> output_sd, "  Rectangle Perimeter < Circle Perimeter"
        else:
            print >> output_sd, "  Rectangle Perimeter > Circle Perimeter"

        print >> output_sd, "\nArea:"

        if rectangle_area == circle_area:
            print >> output_sd, "  Rectangle Area = Circle Area"
        elif rectangle_area < circle_area:
            print >> output_sd, "  Rectangle Area < Circle Area"
        else:
            print >> output_sd, "  Rectangle Area > Circle Area"        

    out_file.close()
    in_file.close()

if __name__ == "__main__":
     main()