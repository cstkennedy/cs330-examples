//Programmer : Thomas Kennedy

import java.util.List;
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


/**
 * This is the Java version of the previous C++ Shapes Inheritance Example.
 *
 * @author Thomas J Kennedy
 */
public class RunShapes {
    /**
     * This is the Program Title.
     */
    private static final String[] PROGRAM_HEADING = {
        "Objects & Inheritance: 2-D Shapes",
        "Thomas J. Kennedy"
    };

    /**
     * The default heading width.
     */
    private static final int H_WIDTH = 38;

    /**
     * This is the main function.
     *
     * @param args args[0] must specify the filename of a valid shape text file
     *
     * @throws CloneNotSupportedException if a Shape subclass can
     *     not be copied.
     */
    public static void main(String[] args)
        throws CloneNotSupportedException
    {
        // Command line argument and File validation
        BufferedReader shapesFile = null;
        try {
            shapesFile = new BufferedReader(new FileReader(args[0]));
        }
        catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Usage: java -jar {jarfile} {inputTextFile}");
            System.exit(1);
        }
        catch (FileNotFoundException e) {
            System.out.println("File (" + args[0] + ") could not be opened.");
            System.exit(2);
        }

        // Print main program heading
        System.out.println(projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH));

        //----------------------------------------------------------------------
        // Examine the ShapeFactory
        //----------------------------------------------------------------------
        System.out.println(heading("Available Shapes", H_WIDTH, '*'));
        System.out.print(ShapeFactory.listKnown());
        System.out.println(horizontalLine('-', H_WIDTH));
        System.out.printf("%2d shapes available.%n", ShapeFactory.numberKnown());
        System.out.println();

        List<Shape> shapes = readShapes(shapesFile);

        // Print all the shapes
        System.out.println(heading("Display All Shapes", H_WIDTH, '*'));
        printShapes(shapes);

        // Using an iterator
        System.out.println(heading("Display Shape Names", 38, '~'));
        printShapeNames(shapes);
        System.out.println();

        System.out.println(heading("Display Largest Shape (Area)", 38, '~'));
        Shape largestShape = findLargestShapeByArea(shapes);
        System.out.println(largestShape);
    }

    /**
     * Read shapes from an input stream
     * and construct an `ArrayList<Shape>` object.
     *
     * @param shapesFile input source
     *
     * @return collection of read-in shapes
     *
     * @throws CloneNotSupportedException if the `ShapeFactory` fails to clone a
     *     model shape
     */
    private static List<Shape> readShapes(BufferedReader shapesFile)
        throws CloneNotSupportedException
    {
        Scanner scanner = new Scanner(shapesFile);
        ArrayList<Shape> collection = new ArrayList<Shape>();

        while (scanner.hasNextLine()) {
            String line   = scanner.nextLine();

            int    sIndex = line.indexOf(';', 0);
            String name   = line.substring(0, sIndex); // [0, sIndex)

            Scanner lineScanner = new Scanner(line.substring(sIndex + 1,
                                                             line.length()));
            // System.out.println(line);
            // System.out.println(name);

            Shape s = ShapeFactory.createShape(name);

            if (s != null) {
                s.read(lineScanner);
                collection.add(s);
            }
            //else {
                // Ignore inLineScanner and line
                // they will cease to exist
                // after this loop iteration
            //}
        }

        return collection;
    }

    /**
     * Print shapes from a `ArrayList<Shape>`
     */
    private static void printShapes(List<Shape> toPrint)
    {
        for (Shape s : toPrint) {
            System.out.println(s);
        }
    }

    /**
     * Print shape names for all `Shape`s in a `ArrayList<Shape>` to a
     * specified output stream
     */
    private static void printShapeNames(List<Shape> toPrint)
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
    private static Shape findLargestShapeByArea(List<Shape> collection)
    {
        Iterator<Shape> it = collection.iterator();

        // The first shape is the largest
        // until I look at more
        Shape largestShape = it.next();

        while (it.hasNext()) {
            Shape s = it.next();

            if (s.area() > largestShape.area()) {
                largestShape = s;
            }
        }

        return largestShape;
    }
}
