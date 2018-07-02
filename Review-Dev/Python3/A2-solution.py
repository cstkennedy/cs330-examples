#! /usr/bin/env python3

import os
import sys
import random
import math


def printHeading(title, outs=sys.stdout):
    print("=" * 40, file=outs)
    print(title.center(40), file=outs)
    print("=" * 40, file=outs)


def main():
    # Prompt the user for prism length
    length = float(input("{:<18}: ".format("Enter the length")))

    # Prompt the user for prism height
    height = float(input("{:<18}: ".format("Enter the height")))

    # Prompt the user for prism width
    width = float(input("{:<18}: ".format("Enter the width")))

    # Prompt the user for sphere radius
    radius = float(input("{:<18}: ".format("Enter the radius")))

    # Validate Input
    if length <= 0 or width <= 0 or height <= 0 or radius <= 0:
        print("\nERROR: All inputs must be strictly greater than 0.")
        sys.exit(2)

    # Open the output file
    out_file = open("A2-cslogin.txt", "w")

    # Perform Calculations
    prism_sa  = 2 * ((length*width) + (length * height) + (width * height))
    prism_vol = length * width * height

    sphere_sa  = 4 * math.pi * (radius ** 2.0)
    sphere_vol = (4 / 3.0) * math.pi * (radius ** 3.0)

    # Output to the screen and file
    for out_sd in [sys.stdout, out_file]:
        printHeading("Summary", out_sd)

        print("Rectangular Prism:", file=out_sd)
        print("  {:<16}: {:>10.2f}".format("Length", length), file=out_sd)
        print("  {:<16}: {:>10.2f}".format("Width", width), file=out_sd)
        print("  {:<16}: {:>10.2f}".format("Height", height), file=out_sd)
        print("  {:<16}: {:>10.2f}".format("S.A.", prism_sa), file=out_sd)
        print("  {:<16}: {:>10.2f}".format("Volume", prism_vol), file=out_sd)

        print("\nSphere:", file=out_sd)
        print("  {:<16}: {:>10.2f}".format("Radius", radius), file=out_sd)
        print("  {:<16}: {:>10.2f}".format("S.A.", sphere_sa), file=out_sd)
        print("  {:<16}: {:>10.2f}".format("Volume", sphere_vol), file=out_sd)

        print("\nSurface Area:", file=out_sd)

        if prism_sa == sphere_sa:
            print("  Prism SA = Sphere SA", file=out_sd)
        elif prism_sa < sphere_sa:
            print("  Prism SA < Sphere SA", file=out_sd)
        else:
            print("  Prism SA > Sphere SA", file=out_sd)

        print("\nVolume:", file=out_sd)

        if prism_vol == sphere_vol:
            print("  Prism Volume = Sphere Volume", file=out_sd)
        elif prism_vol < sphere_vol:
            print("  Prism Volume < Sphere Volume", file=out_sd)
        else:
            print("  Prism Volume > Sphere Volume", file=out_sd)

    out_file.close()


if __name__ == "__main__":
    main()
