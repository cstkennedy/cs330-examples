import java.util.Scanner;

/**
 * Generate the Fibonacci Sequence to the n-th number.
 * 1 1 2 3 5 8 13 21 34...
 * <p>
 * The user must enter a number no smaller than 3 and
 * no greater than 20
 */
public class Fibonacci {
    /**
     *
     */
    public static void main(String argv[])
    {
        Scanner input = new Scanner(System.in);        
        
        int     index = 3; // Desired length of sequence
        
        // Fibonaccci
        int     fm2   = 1; // n-2 (previous previous) fibonacci number
        int     fm1   = 1; // n-1 (previous) fibonacci number
        int     f     = 0; // current fibonacci number

        // Prompt the user
        System.out.print("Generate how many numbers? ");
        index = input.nextInt();

        // Print a blank line
        System.out.println();

        /*
        Note what happens if we do not check the
        index entered by the user

        De-Morgan's Law
        !(index >= 3 && index <= 20)
        !(index >= 3) || !(index <= 20)
        (index < 3 || index > 20)
        */    
        if(index < 3 || index > 20){       
            // Error Message
            System.out.printf("%3d is not between 3 and 20\n", index);
            
            // Exit with an error state
            System.exit(1);
        }        

        // Initial output
        System.out.printf("%2d: %10d\n", 1, fm2);
        System.out.printf("%2d: %10d\n", 2, fm1);

        // The first 2 numbers were already output
        for(int i = 3; i <= index; i++) {
            f   = fm1 + fm2;
            fm2 = fm1;
            fm1 = f;

            System.out.printf("%2d: %10d\n", i, f);
        }
    }
}

