#! /usr/bin/env python                          

# Thomas Kennedy
# February 2014
# Assignment 2 Solution

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
    #Prompt the user for prism length
    length = float( raw_input( "{:<18}: ".format( "Enter the length" ) ) )

    #Prompt the user for prism height
    height = float( raw_input( "{:<18}: ".format( "Enter the height" ) ) )

    #Prompt the user for prism width
    width = float( raw_input( "{:<18}: ".format( "Enter the width"  ) ) )     

    #Prompt the user for sphere radius
    radius = float( raw_input( "{:<18}: ".format( "Enter the radius" ) ) )

    #Validate Input
    if length <= 0 or width <= 0 or height <=0 or radius <= 0 :
        print "\nERROR: All inputs must be strictly greater than 0."
        sys.exit( 2 )

    #open the output file
    out_file = open( "A2-cslogin.txt", "w")

    #Perform Calculations
    prism_sa   = 2 * ( (length*width) + (length*height) + (width*height) );
    prism_vol  = length * width * height;

    sphere_sa  = 4 * math.pi * ( radius ** 2.0 )
    sphere_vol = (4 / 3.0) * math.pi * ( radius ** 3.0 )
          
    #Output to the screen and file
    for output_sd in [sys.stdout, out_file]:
        printHeading( "Summary", output_sd )

        print >> output_sd, "Rectangular Prism:" 
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Length", length    )    
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Width" , width     )  
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Height", height    )  
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "S.A."  , prism_sa  )  
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Volume", prism_vol )

        print >> output_sd, "\nSphere:" 
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Radius", radius    )
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "S.A."  , sphere_sa ) 
        print >> output_sd, "  {:<16}: {:>10.2f}".format( "Volume", sphere_vol)

        print >> output_sd, "\nSurface Area:"

        if prism_sa == sphere_sa:
            print >> output_sd, "  Prism SA = Sphere SA"
        elif prism_sa < sphere_sa:
            print >> output_sd, "  Prism SA < Sphere SA"
        else:
            print >> output_sd, "  Prism SA > Sphere SA"

        print >> output_sd, "\nVolume:"

        if prism_vol == sphere_vol:
            print >> output_sd, "  Prism Volume = Sphere Volume"
        elif prism_vol < sphere_vol:
            print >> output_sd, "  Prism Volume < Sphere Volume"
        else:
            print >> output_sd, "  Prism Volume > Sphere Volume"        

    out_file.close()

if __name__ == "__main__":
     main()