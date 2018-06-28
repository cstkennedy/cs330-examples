#! /usr/bin/env python3     

# Programmer : Thomas Kennedy      
# Date       : 5 March 2014   
# Description: Lab Assignment 0 Solution

import os
import sys
import datetime

# Global Variables
W_WIDTH = 80; # Width of the terminal window

#
# Print a centered and styled heading
#
def printHeading(title, width=W_WIDTH, outs=sys.stdout):
    print >> outs, "=" * width
    print >> outs,title.center(width)
    print >> outs,"=" * width

#
# Get the student's username from his or her first and last name
#
def getUsername( first, last ):
    return (first[0] + ( last[:7] if len(last) > 7 else last ) ).lower()

#
# Main Function
#
def main():
    user_response = "N";     # Stores the student's response

    # Print a fancy heading
    printHeading( "CS150: Spring 2014 - Lab Assignment 1")

    # Get the student's first and last name
    while user_response.upper() != "Y":    
        # Reset user response for each round.
        user_response = "Q"    

        #Read the user's first and last name
        first_name = raw_input("Enter your first name: ").split()[0].title()
        last_name  = raw_input("Enter your last name : ").split()[0].title()

        # Validate the student's name was entered correctly
        while user_response.upper() not in ["Y","N"]:        
            user_response = raw_input("Confirm: Is your name {} {}? (Y/N): ".format( first_name, last_name ) )        

        # If the student's name is correct, stop the loop, otherwise print a message and go back to the top.
        if user_response == "N":        
            print "Input Error: Please try again."

    # Print a line to the screen
    print "*" * W_WIDTH

    # Get the student's birth year.
    birth_year = int( raw_input("Enter the year you were born: ") )

    # Print a line to the screen
    print "*" * W_WIDTH

    # Calculate and save the username
    username = getUsername( first_name, last_name );

    # Calculate and save the age
    age = int( datetime.date.today().strftime("%Y") ) -  birth_year

    print "Your CS username is most likely {}".format( username )
    print ("You are probably turning {} this year" + ("... that's unusual." if age < 16 or age > 50 else ".")).format( age )

    # File Output
    filename = username + "-lab-assignment-0.txt";
    print "Writing output to {}.\nYou can find it in the same folder as the PY.".format( filename )

    #open the output file
    try:
        out_file = open(filename, "w");
    except:
        print "\nError: Could not open file.\n"
        sys.exit(1)
    
    printHeading(first_name + "'s Lab Assignment 1", W_WIDTH, out_file);

    print >> out_file, "{:<20}: {:<20}".format( "First Name",     first_name )
    print >> out_file, "{:<20}: {:<20}".format( "Last Name",      last_name )
    print >> out_file, "{:<20}: {:<20}".format( "Today's Date",   datetime.date.today().strftime("%d %A %Y") )
    print >> out_file, "{:<20}: {:<20}".format( "Estimated Age:", age )

    print >>out_file, "*" * W_WIDTH

    out_file.close()

if __name__ == "__main__":
     main()