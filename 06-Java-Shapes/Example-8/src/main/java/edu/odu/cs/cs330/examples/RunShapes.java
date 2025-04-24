package edu.odu.cs.cs330.examples;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;

import java.util.List;
import java.util.ArrayList;

import java.util.Comparator;
import java.util.Iterator;
import java.util.Spliterator;
import java.util.Spliterators;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

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
        //----------------------------------------------------------------------
        // Command line argument and File validation
        //----------------------------------------------------------------------
        BufferedReader shapesFile = null;
        try {
            shapesFile = new BufferedReader(new FileReader(args[0]));
        }
        catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Usage: java -jar {jarfile} {inputTextFile}");
            System.exit(1);
        }
        catch (FileNotFoundException e) {
            System.out.printf("File (%s) could not be opened.%n", args[0]);
            System.exit(2);
        }

        //----------------------------------------------------------------------
        // Print main program heading
        //----------------------------------------------------------------------
        System.out.println(projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH));

        //----------------------------------------------------------------------
        // Examine the ShapeFactory
        //----------------------------------------------------------------------
        System.out.println(heading("Available Shapes", H_WIDTH, '*'));
        System.out.print(ShapeFactory.listKnown());
        System.out.println(horizontalLine('-', H_WIDTH));
        System.out.printf("%2d shapes available.%n", ShapeFactory.numberKnown());
        System.out.println();

        //----------------------------------------------------------------------
        // Get list of shapes from file
        //----------------------------------------------------------------------
        List<Shape> shapes = readShapes(shapesFile);

        //----------------------------------------------------------------------
        // Print all the shape names
        //----------------------------------------------------------------------
        System.out.println(heading("Display Shape Names", H_WIDTH, '*'));
        for (Shape shp : shapes) {
            System.out.println(shp.name());
        }
        System.out.println();

        //----------------------------------------------------------------------
        // Print all the shapes
        //----------------------------------------------------------------------
        System.out.println(heading("Display Shapes", H_WIDTH, '~'));
        for (Shape shp : shapes) {
            System.out.println(shp);
        }
        System.out.println();

        //----------------------------------------------------------------------
        // Find and print the largest shape based on area
        //----------------------------------------------------------------------
        System.out.println(heading("Display Largest Shape (Area)", H_WIDTH, '~'));
        Shape largestShape = shapes.parallelStream()
                           .max(Comparator.comparing(Shape::area))
                           .get();
        System.out.println(largestShape);

        //----------------------------------------------------------------------
        // Find and print the smallest shape based on perimeter
        //----------------------------------------------------------------------
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
        /*
        List<Shape> collection = new ArrayList<>();

        Iterator<Shape> it = new ShapeIterator(shapesFile);

        while (it.hasNext()) {
            Shape shp = it.next();

            collection.add(shp);
        }

        return collection;
        */
        Iterator<Shape> it = new ShapeIterator(shapesFile);
        Spliterator<Shape> splitIt = Spliterators.spliteratorUnknownSize(
            it,
            Spliterator.ORDERED
        );
        Stream<Shape> stream = StreamSupport.stream(splitIt, false); // false -> not parallel
        List<Shape> collection = stream.collect(Collectors.toList());

        return collection;
    }
}
