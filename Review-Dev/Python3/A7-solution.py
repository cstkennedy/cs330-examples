#! /usr/bin/env python                          

# Thomas Kennedy
# March 2014
# Assignment 7 Solution

import os
import sys
import random
import math

#
# Print a centered and styled heading
#
def printHeading(title, width=40, outs=sys.stdout):
    print("=" * width, file=outs)
    print(title.center(width), file=outs)
    print("=" * width, file=outs)

#
# Determine the letter grade
#
def determineLetterGrade( to_classify ):
    to_return = 'F'

    if to_classify >= 90:
        to_return = 'A'
    elif to_classify >= 80:
        to_return = 'B'
    elif to_classify >= 70:
        to_return = 'C'
    elif to_classify >= 60:
        to_return = 'D'

    return to_return

#Main Function
def main():    
    #Variable Declarations
    grades        = []   #List of grades
    num_grades    = 0    #Number of grades to enter

    grade_min     = 9001 #Lowest grade
    grade_max     = 0    #Highest grade

    grade_avg     = 0    #Average Grade
    grade_var     = 0    #Variance in Grades
    grade_std_dev = 0    #Standard Deviation in Grades

    num_grades = int( input("How many grades will be entered?: ") )

    #Read all the grades and perform initial analysis
    for i in range(0, num_grades):
        #Add the grade to the list
        grades.append( float( input("Enter Grade # {:>3}: ".format((i + 1)) ) ) )

        #Check if the new grade is a minimum
        if grades[i] < grade_min:
            grade_min = grades[i]
        
        #Check if the new grade is a maximum
        #this is a seperate and distinct conditional block
        if grades[i] > grade_max:
            grade_max = grades[i]
        
        #update the running sum
        #consider the use of average as storage
        grade_avg += grades[i]

        #update the running sum of squares --i.e., the
        #sum of squares of all grades
        #consider the use of variance as storage
        grade_var += ( grades[i] ** 2.0 )
    
    #compute the average
    grade_avg /= num_grades

    #compute the variance
    grade_var = (grade_var / num_grades ) - pow( grade_avg, 2.0 )

    #compute standard deviation
    grade_std_dev = math.sqrt( grade_var )

    #determine the median - in the case of 2 medians, take the lower median
    grade_median = sorted(grades)[ int(len(grades)/2)-1 ]

    printHeading("Grade Listing")

    #Output the grades in table form
    print("{:<8}{:>16}{:>16}".format("Grade #", "Score", "Letter"))
    print("-" * 40)

    for i in range(0, len(grades)):
        print("{:<8}{:>16.2f}{:>16}".format( (i+1), grades[i], determineLetterGrade( grades[i] ) ))

    #Print a blank line
    print() 

    #print the grade summary
    printHeading("Grade Summary")
    print("{:<16}:{:>8.2f} ({})".format( "Minimum",   grade_min,    determineLetterGrade(grade_min)   ))
    print("{:<16}:{:>8.2f} ({})".format( "Maximum",   grade_max,    determineLetterGrade(grade_max)   ))
    print("{:<16}:{:>8.2f} ({})".format( "Average",   grade_avg,    determineLetterGrade(grade_avg)   ))
    print("{:<16}:{:>8.2f} ({})".format( "Median",    grade_median, determineLetterGrade(grade_median)))
    print()
    print("{:<16}:{:>8.2f}".format( "Variance",  grade_var ))
    print("{:<16}:{:>8.2f}".format( "Std. Dev.", grade_std_dev ))

if __name__ == "__main__":
     main()