import java.util.Scanner;

/**
 * A basic keyboard Input Stream example
 */
class KeyboardInput {    
    /**
     * Java main function.
     *
     * @param argv command line arguments. By convention
     *     this is usually "args" in Java.
     */
    public static void main(String[] argv)
    {
        Scanner inputStream          = new Scanner(System.in);

        int     inputInt             = 0;
        double  inputDouble          = 0;
        char    inputChar            = '\0';
        boolean inputBoolean         = false;

        String  inputStringNoSpaces  = null;
        String  inputStringWholeLine = null;

        System.out.print("Enter an Integer: ");
        inputInt = inputStream.nextInt();

        System.out.print("Enter a Double: ");
        inputDouble = inputStream.nextDouble();

        System.out.print("Enter a Character: ");
        inputChar = (inputStream.next()).charAt(0);

        System.out.print("Enter a Boolean: ");
        inputBoolean = inputStream.nextBoolean();

        System.out.print("Enter an String (no spaces): ");
        inputStringNoSpaces = inputStream.next();

        // Same as the C++ getline
        // Strip the trailing newline from the previous read
        inputStream.nextLine();

        System.out.print("Enter a String (with spaces): ");
        inputStringWholeLine = inputStream.nextLine();

        System.out.println(); // cout << "\n";
        System.out.println("You Entered:"); // cout << "You Entered:" << "\n";
        System.out.format("  Item %2d: %d\n",    1, inputInt);
        System.out.format("  Item %2d: %4.2f\n", 2, inputDouble);
        System.out.format("  Item %2d: %s\n",    3, inputChar);
        System.out.format("  Item %2d: %b\n",    4, inputBoolean);
        System.out.format("  Item %2d: %s\n",    5, inputStringNoSpaces);
        System.out.format("  Item %2d: %s\n",    6, inputStringWholeLine);
    }
}