#! /usr/bin/env python3

def main():
    """
    Generate the Fibonacci Sequence to the n-th number.
    1 1 2 3 5 8 13 21 34...
    <p>
    The user must enter a number no smaller than 3 and
    no greater than 20
    """    
    
    index = 3 # Desired length of sequence
    
    # Fibonaccci
    fm2   = 1 # n-2 (previous previous) fibonacci number
    fm1   = 1 # n-1 (previous) fibonacci number
    f     = 0 # current fibonacci number

    # Prompt the user
    index = input("Generate how many numbers? ")
    index = int(index) # loosely typed

    # Print a blank line
    print()
    
    # Note what happens if we do not check the
    # index entered by the user

    # De-Morgan's Law
    # !(index >= 3 && index <= 20)
    # !(index >= 3) || !(index <= 20)
    # (index < 3 || index > 20)
      
    if index < 3 or index > 20:       
        # Error Message
        print("{:3d} is not between 3 and 20\n", index)
        
        # Exit with an error state
        exit(1)
        

    # Initial output
    print("{:>2d}: {:10d}".format(1, fm2))
    print("{:>2d}: {:10d}".format(2, fm1))

    # Fhe first 2 numbers were already output
    for i in range(3, (index + 1)):
        f   = fm1 + fm2
        fm2 = fm1
        fm1 = f

        print("{:>2d}: {:10d}".format(i, f))


if __name__ == "__main__":
    try:
        main()
    except:
        pass

