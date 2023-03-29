package edu.odu.cs.cs330.examples;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;

import java.util.List;
import java.util.ArrayList;

import java.util.Comparator;
import java.util.Iterator;

import edu.odu.cs.tkennedy.utilities.Utilities;
import static edu.odu.cs.tkennedy.utilities.Utilities.heading;
import static edu.odu.cs.tkennedy.utilities.Utilities.horizontalLine;
import static edu.odu.cs.tkennedy.utilities.Utilities.projectHeading;

// import edu.odu.cs.cs330.examples.shapes.*;
import edu.odu.cs.cs330.examples.shapes.Shape;
import edu.odu.cs.cs330.examples.shapes.ShapeFactory;

import edu.odu.cs.cs330.examples.shapes.io.ShapeIterator;

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
        System.out.println(heading("Display Shape Names", H_WIDTH, '*'));
        //printShapeNames(shapes);

        for (Shape shp : shapes) {
            System.out.println(shp.name());
        }

        /*
        shapes.stream()
            .map(Shape::name)
            .forEach(System.out::println);
        */

        System.out.println();
        System.out.println(heading("Display Shapes", H_WIDTH, '~'));
        for (Shape shp : shapes) {
            System.out.println(shp);
        }
        /*
        shapes.stream()
            .forEach(System.out::println);
        */

        System.out.println();

        System.out.println(heading("Display Largest Shape (Area)", H_WIDTH, '~'));
        Shape largestShape = shapes.parallelStream()
                           .max(Comparator.comparing(Shape::area))
                           .get();
        System.out.println(largestShape);

        System.out.println(heading("Display Smallest Shape (Perimeter)", H_WIDTH, '~'));
        Shape smallestShape = shapes.parallelStream()
                            .min(Comparator.comparing(Shape::perimeter))
                            .get();
        System.out.println(smallestShape);
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
        List<Shape> collection = new ArrayList<>();

        Iterator<Shape> it = new ShapeIterator(shapesFile);

        while (it.hasNext()) {
            Shape shp = it.next();

            if (shp != null) {
                collection.add(shp);
            }
        }

        return collection;
    }
}
