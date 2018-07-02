#! /usr/bin/env python3


import os
import sys
import random
import math

from itertools import groupby

# Constants
W_WIDTH = 80      # Width of the terminal window
NUM_READINGS = 4  # Number of temperature readings stored by each station
HEADINGS = []


def printHeading(title, width=40, outs=sys.stdout):
    """
    Print a centered and styled heading
    """

    print("=" * width, file=outs)
    print(title.center(width), file=outs)
    print("=" * width, file=outs)


def computeAverageReading( to_update ):
    """
    Set the average temperature for one station
    """

    to_update["average_temperature"] = sum( to_update["readings"]
                                     / len(to_update["readings"] ))


def printWeatherHeading():
    """
    Print the heading for the Weather Station table
    """

    print("-" * 74)

    print("{:<4}| {:<10}| {:<10}| {:<32}| {:<10}".format( "ID", 
                                                      "Latitude", 
                                                      "Longitude",
                                                      "Readings",
                                                      "Average"
                                                    ))
    print("-" * 74)


#
# Print 1 Weather Station
#
def printWeatherStation(to_print):    
    print("{:>4}| {:>10.2f}| {:>10.2f}| {:>32}| {:>10.2f}".format( to_print["id"],
                                                                   to_print["location"]["latitude"],
                                                                   to_print["location"]["longitude"],
                                                                   "".join( ["{:>8.2f}".format(x) for x in to_print["readings"] ] ),
                                                                   to_print["average_temperature"]
                                                                 ))

#Main Function
def main():    
    #prompt the user for the file name
    file_name = str( input("Enter input filename: ") )

    #open the input file
    try:        
        in_file = open( file_name, "r")
    except:
        print("\nError: Could not open file.\n")
        sys.exit(1)

    #Read the entire file and ignore the first line
    stations = [ { "location":  {"latitude": float(line[0]), "longitude": float(line[1])}, 
                   "readings":[ float(x) for x in line[2:] ] 
                 } for line in [line.strip().split() for line in in_file.readlines()][1:]
               ]

    #print the weather station table
    printWeatherHeading()
    for i in range(0, len( stations ) ):
        #set the id
        stations[i]["id"] = (i+1)

        #compute the average temperature
        computeAverageReading( stations[i] )

        printWeatherStation( stations[i] )

    print("-" * 74)

    #sort the stations by average temperature
    stations = sorted( stations, key = lambda x: x["average_temperature"] ) 

    #Print a summary of the station lowest average
    print("{:<12}: {:>8.2f} ( Station {} )".format( "Lowest Avg",  
                                                    stations[0]["average_temperature"], 
                                                    stations[0]["id"] 
                                                  ))

    #Print a summary of the station lowest average
    print("{:<12}: {:>8.2f} ( Station {} )".format( "Highest Avg",  
                                                    stations[-1]["average_temperature"], 
                                                    stations[-1]["id"] 
                                                  ))
if __name__ == "__main__":
    main()