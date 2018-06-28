#! /usr/bin/env python                          

# Thomas Kennedy
# February 2014
# Assignment 5 Solution

import os
import sys
import random
import math

#Constants residential customers
RES_BILL_PROC_FEES    =  4.50
RES_BASIC_SERV_COST   = 20.00
RES_COST_PREM_CHANNEL =  8.00

#Constants business customers
BUS_BILL_PROC_FEES    = 15.00
BUS_BASIC_SERV_COST   = 75.00
BUS_BASIC_CONN_COST   = 5.00

#
# Print a styled and centered heading
#
def printHeading(title, width=40, outs=sys.stdout):
    print >> outs, "=" * width
    print >> outs,title.center(width)
    print >> outs,"=" * width


#Main Function
def main():
    #Variable declaration and intialization
    total_residential = 0    # Total Profit from residential customers 
    total_business    = 0    # Total Profit from business customers 

    num_residential   = 0    # Number of residential customers
    num_business      = 0    # Number of business customers

    try:
        #open the input file
        in_file = open( "A4-input.txt", "r")
    except:
        print "\nError: Could not open file.\n"
        sys.exit(1)

    #print Heading
    printHeading( "COMPUTING THE CABLE BILL", 40 )

    #Loop Until the End of the File
    for line in in_file:

        #create a list of items on the line-after stripping all whitespace
        line = [x.strip() for x in line.split()]

        #Read the account number
        account_number = line[0]

        #Convert Lowercase Letter to Uppercase Letter
        account_type = line[1].upper()

        print  "\nCurrently Modifying:"
        print " {:<15}: {:<20}".format("Acct #", account_number)
        print " {:<15}:".format("Acct Type"),

        if account_type == "R":
            print  "Residential\n"  

            num_premium_ch = int( raw_input("Enter # of premium channels: ") )

            account_balance = ( RES_BILL_PROC_FEES + RES_BASIC_SERV_COST 
                             + ( num_premium_ch * RES_COST_PREM_CHANNEL )  
                              )

            total_residential += account_balance      
            num_residential += 1       

            print "\n{:<15}: $ {:<.2f}".format("New Balance", account_balance )

        elif account_type == "B":
            print "Business\n\n"               

            num_serv_conn = int( raw_input("Enter # of service connections: " ) )


            #Calculate Amount Due
            account_balance = ( BUS_BILL_PROC_FEES + BUS_BASIC_SERV_COST
                               +( num_serv_conn  * BUS_BASIC_CONN_COST )
                              )

            total_business += account_balance
            num_business += 1

            print "\n{:<15}: $ {:<.2f}".format("New Balance", account_balance )
        
        else:
            print "Invalid\n" 
        
        print "-" * 40
    

    printHeading( "PROFIT SUMMARY", 40 )

    print "{:<18}: {:>8.2f}".format("Residential Accts", num_residential)
    print "{:<18}: {:>8.2f}".format("Business Accts"    , num_business)
    print
    print "{:<18}: {:>8.2f}".format("Residential Profit", total_residential)
    print "{:<18}: {:>8.2f}".format("Business Profit"   , total_business)
    print "{:<18}: {:>8.2f}".format("Total Profit",( total_business + total_residential ) )

    in_file.close()

if __name__ == "__main__":
     main()