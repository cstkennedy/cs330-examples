//Programmer : Thomas Kennedy

import java.util.ArrayList;
import java.util.Iterator;
import java.util.Scanner;
import java.io.BufferedReader;
import java.io.FileReader;
import java.io.FileNotFoundException;

import utilities.Utilities;
import static utilities.Utilities.projectHeading;
import static utilities.Utilities.heading;
import static utilities.Utilities.projectHeading;
import static utilities.Utilities.horizontalLine;

import shapes.*;

public class RunShapes {
    static final String[] PROGRAM_HEADING = {
        "Objects & Inheritance: 2-D Shapes",
        "Thomas Kennedy"
    };  ///< Program Title

    public static void main(String[] args)
    {
        BufferedReader shapesFile = null;
        try {
            // index is zero because java does command line arguments
            // the wrong way
            shapesFile = new BufferedReader(new FileReader(args[0]));
        }
        catch(ArrayIndexOutOfBoundsException e) {
            System.out.println("Usage: java -jar {jarfile} {inputTextFile}");
            System.exit(1);
        }
        catch(FileNotFoundException e) {
            System.out.println("File (" + args[0] + ") could not be opened.");
            System.exit(2);
        }

        // Print main program heading
        System.out.println(projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH));

        // Examine the ShapeFactory
        System.out.println(heading("Available Shapes", 38, '*'));

        // List the available shapes
        System.out.print(ShapeFactory.listKnown());
        System.out.println(horizontalLine('-', 38));
        System.out.printf("%2d shapes available.\n", ShapeFactory.numberKnown());

        System.out.println();

        // Create 5 "Random" Shapes
        int              size   = 0; // original size of shapes container

        Scanner scanner = new Scanner(shapesFile);

        ArrayList<Shape> shapes = readShapes(scanner);

        // Print all the shapes
        System.out.println(heading("Display All Shapes", 38, '*'));
        printShapes(shapes);

        // Using an iterator
        System.out.println();
        System.out.println(heading("Display Shape Names", 38, '~'));
        printShapeNames(shapes);
        System.out.println();

        System.out.println(heading("Display Largest Shape (Area)", 38, '~'));
        Shape largestShape = findLargestShapeByArea(shapes);

        //cout << largestShape << "\n"; // oops again
        //cout << *largestShape << "\n";
        //cout << *(*it) << "\n"; // skip the temporary Shape*
        System.out.println(largestShape);
    }

    /**
     * Add a Shape iff it is not null
     *
     * @param shapes container of Shapes
     * @param toAdd shape "reference" to add
     *
     * @deprecated
     */
    private static void addShape(ArrayList<Shape> shapes, Shape toAdd)
    {
        if (toAdd != null) {
            shapes.add(toAdd);
        }
    }

    /**
     * Read shapes from an input stream
     * and construct a `ArrayList<Shape>` object
     */
    private static ArrayList<Shape> readShapes(Scanner scanner)
    {
        ArrayList<Shape> collection = new ArrayList<Shape>();

        while (scanner.hasNextLine()) {
            String  line = scanner.nextLine();

            //String name = inLineScanner.next();
            int    sIndex = line.indexOf(';', 0);
            String name   = line.substring(0, sIndex); // [0, sIndex)

            Scanner lineScanner = new Scanner(
                line.substring(sIndex + 1, line.length())
            );

            //System.out.println(line);
            //System.out.println(name);

            Shape s = ShapeFactory.createShape(name);

            if (s != null) {
                s.read(lineScanner);

                // Let us skip the call to addShape
                collection.add(s);
            }
            else {
                // Ignore inLineScanner and line
                // they will cease to exist
                // after this loop iteration
            }
        }

        return collection;
    }

    /**
     * Print shapes from a `ArrayList<Shape>`
     */
    private static void printShapes(ArrayList<Shape> toPrint)
    {
        for (Shape s : toPrint) {
            System.out.println(s);
        }
    }

    /**
     * Print shape names for all `Shape`s in a `ArrayList<Shape>` to a
     * specified output stream
     */
    static void printShapeNames(ArrayList<Shape> toPrint)
    {
        // C++ Container<Shape*>::iterator it = shapes.begin()
        Iterator<Shape> it = toPrint.iterator();

        // C++ while (it != toPrint.end())
        while (it.hasNext()) {
            Shape s = it.next();
            System.out.println(s.name());
        }
    }

    /**
     * Find the largest `Shape` (by area) in a `ArrayList<Shape>`
     *
     * @return an iterator at the position of the largest `Shape`
     */
    static Shape findLargestShapeByArea(ArrayList<Shape> collection)
    {
        Iterator<Shape> it = collection.iterator();

        // The first shape is the largest
        // until I look at more
        Shape largestShape = it.next();

        while (it.hasNext()) {
            // std::cerr << "\n" << *(*it) << "\n"; // C++ debugging

            Shape s = it.next();

            if (s.area() > largestShape.area()) {
                largestShape = s;
            }
        }

        return largestShape;
    }
}
