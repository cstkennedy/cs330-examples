package edu.odu.cs.cs330.examples;

import java.util.Scanner;
import java.io.FileReader;
import java.io.BufferedReader;
import java.io.FileNotFoundException;

import java.util.Comparator;

import java.util.List;
import java.util.ArrayList;

import edu.odu.cs.tkennedy.utilities.Utilities;
import static edu.odu.cs.tkennedy.utilities.Utilities.heading;
import static edu.odu.cs.tkennedy.utilities.Utilities.projectHeading;
import static edu.odu.cs.tkennedy.utilities.Utilities.horizontalLine;

import edu.odu.cs.cs330.examples.shapes.*;

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
     * @param args[0] input filename
     *
     * @throws CloneNotSupportedException if a shapes subclass can
     *     not be copied.
     */
    public static void main(String[] args)
        throws CloneNotSupportedException
    {
        BufferedReader shapesFile = null;
        try {
            // index is zero because java does command line arguments
            // the wrong way
            shapesFile = new BufferedReader(new FileReader(args[0]));
        }
        catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Usage: java -jar {jarfile} {inputTextFile}");
            System.exit(1);
        }
        catch (FileNotFoundException e) {
            System.out.println("File (" + args[0] + ") could not be opened.");
            System.exit(1);
        }

        // Print main program heading
        System.out.println(projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH));

        /*
         * What happens when the number of shapes is non-trivial?
         *
         * Suppose we were to expand our Shape hierarchy to include
         * the following shapes:
         *   - Isosceles Triangle
         *   - Circle
         *   - Ellipse
         *   - Rectangle
         *   - Square
         *   - Rhombus
         *   - Parallelogram
         *   - Kite
         *   - Generalized Polygon
         *
         * How would we manage the addition of new Shapes?
         *
         * A common approach is to make use of the Factory Model.
         * This Model exists in a number of languages--e.g.:
         *   - C++
         *   - Java
         *   - Python
         *   - PHP
         *   - C#
         *
         * A class that contains static members is created.
         * As new classes are created, the Factory Class is
         * updated.
         *
         * In this example, our factory class is called ShapeFactory
         * The ShapeFactory could be designed as a singleton class.
         * Our ShapeFactory is simply a tracker--i.e., records are static
         * and will be updated manually at compile time.
         *
         * The Singleton Class implementation may be discussed at a
         * later date
         */

        // Examine the ShapeFactory
        System.out.println(heading("Available Shapes", H_WIDTH, '*'));

        // List the available shapes
        System.out.print(ShapeFactory.listKnown());
        System.out.println(horizontalLine('-', H_WIDTH));
        System.out.printf("%2d shapes available.%n",
                          ShapeFactory.numberKnown());

        System.out.println();

        Scanner scanner = new Scanner(shapesFile);
        List<Shape> shapes = readShapes(scanner);

        // Print all the shapes
        System.out.println(heading("Display All Shapes", H_WIDTH, '*'));
        //printShapes(shapes);
        shapes.stream().map(Shape::name).forEach(System.out::println);

        // Using a stream
        System.out.println();
        System.out.println(heading("Display Shape Names", H_WIDTH, '~'));
        shapes.stream().forEach(System.out::println);
        System.out.println();

        System.out.println(heading("Display Largest Shape (Area)", H_WIDTH, '~'));
        //Shape largestShape = findLargestShapeByArea(shapes);
        Shape largestShape = shapes.stream()
                                .max(Comparator.comparing(Shape::area))
                                .get();
        System.out.println(largestShape);

        System.out.println(heading("Display Smallest Shape (Perimeter)", H_WIDTH, '~'));
        Shape smallestShape = shapes.stream()
                                .min(Comparator.comparing(Shape::perimeter))
                                .get();
        System.out.println(smallestShape);
    }

    /**
     * Read shapes from an input stream
     * and construct an `ArrayList<Shape>` object.
     *
     * @param scanner input source
     *
     * @return collection of read-in shapes
     *
     * @throws CloneNotSupportedException if the `ShapeFactory` fails to clone a
     *     model shape
     */
    private static List<Shape> readShapes(Scanner scanner)
        throws CloneNotSupportedException
    {
        List<Shape> collection = new ArrayList<Shape>();

        while (scanner.hasNextLine()) {
            String line   = scanner.nextLine();
            int    sIndex = line.indexOf(';', 0);
            String name   = line.substring(0, sIndex); // [0, sIndex)

            Scanner lineScanner = new Scanner(line.substring(sIndex + 1,
                                                             line.length()));

            Shape shp = ShapeFactory.createShape(name);

            if (shp != null) {
                shp.read(lineScanner);
                collection.add(shp);
            }
        }

        return collection;
    }
}
