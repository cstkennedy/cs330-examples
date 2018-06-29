#! /usr/bin/env python


import os
import sys
import random


#
# Print a stylized heading
#
def printHeading(title):
    print("=" * 40)
    print(title.center(40))
    print("=" * 40)


# Main Function
def main():
    printHeading("CS 150 Lab Spring 2014")

    # Read num_2 and num_2
    num_1 = int(input("Enter an Integer: "))
    num_2 = int(input("Enter an Integer: "))

    # Perform Calcualations
    i_addition       = num_1 + num_2
    i_subtraction    = num_1 - num_2
    i_multiplication = num_1 * num_2
    i_division       = num_1 / num_2
    i_modulus        = num_1 % num_2  # % is the remainder from int division

    printHeading("Integer Arithmetic Results")
    print("{:<18}: {:>20}".format("Addition", i_addition))
    print("{:<18}: {:>20}".format("Subtraction", i_subtraction))
    print("{:<18}: {:>20}".format("Multiplication", i_multiplication))
    print("{:<18}: {:>20}".format("Division", i_division))
    print("{:<18}: {:>20}".format("Modulus", i_modulus))

    # Print a blank line
    print("")

    num_3 = float(input("Enter a Real Number: "))
    num_4 = float(input("Enter a Real Number: "))

    d_addition       = num_3 + num_4
    d_subtraction    = num_3 - num_4
    d_multiplication = num_3 * num_4
    d_division       = num_3 / num_4

    printHeading("Real Arithmetic Results")
    print("{:<18}: {:>20.2f}".format("Addition", d_addition))
    print("{:<18}: {:>20.2f}".format("Subtraction", d_subtraction))
    print("{:<18}: {:>20.2f}".format("Multiplication", d_multiplication))
    print("{:<18}: {:>20.2f}".format("Division", d_division))

    print("")
    temp_f = float(input("Enter a Temperature (F): "))

    temp_c = (5.0/9) * (temp_f - 32)
    temp_k = temp_c + 273.15

    printHeading("Temperature Results")
    print("{:<18}: {:>20.2f}".format("Farenheit", temp_f))
    print("{:<18}: {:>20.2f}".format("Celcius", temp_c))
    print("{:<18}: {:>20.2f}".format("Kelvin", temp_k))

if __name__ == "__main__":
    main()
