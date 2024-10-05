package edu.odu.cs.cs330.examples;

import java.io.FileNotFoundException;
import java.io.IOException;

import java.util.List;
import java.util.Comparator;

import edu.odu.cs.tkennedy.utilities.Utilities;
import static edu.odu.cs.tkennedy.utilities.Utilities.heading;
import static edu.odu.cs.tkennedy.utilities.Utilities.horizontalLine;
import static edu.odu.cs.tkennedy.utilities.Utilities.projectHeading;

import edu.odu.cs.cs330.examples.shapes.Shape;
import edu.odu.cs.cs330.examples.shapes.ShapeFactory;
import edu.odu.cs.cs330.examples.shapes.io.ShapeParser;

/**
 * This is the Java version of the Shapes Inheritance Example.
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
     * Collect all information about the ShapeFactory (e.g., supported shapes)
     * ahead of time.
     */
    private static final String SHAPE_FACTORY_SUMMARY = String.join(
        System.lineSeparator(), // replace '\n'
        heading("Available Shapes", H_WIDTH, '*'),
        ShapeFactory.listKnown(),
        horizontalLine('-', H_WIDTH),
        String.format("%2d shapes available.%n", ShapeFactory.numberKnown())
    );

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
        List<Shape> shapes = null;
        try {
            shapes = ShapeParser.readShapesFromFile(args[0]);
        }
        catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Usage: java -jar {jarfile} {inputTextFile}");
            System.exit(1);
        }
        catch (FileNotFoundException e) {
            System.out.printf("File (%s) could not be opened.%n", args[0]);
            System.exit(2);
        }
        catch (IOException e) {
            System.out.printf("File (%s) could not be read.%n", args[0]);
            System.exit(3);
        }

        if (shapes.size() < 1) {
            System.out.printf("File (%s) did not contain any valid shapes.", args[0]);
            System.exit(4);
        }

        //----------------------------------------------------------------------
        // Print main program heading
        //----------------------------------------------------------------------
        System.out.println(projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH));

        //----------------------------------------------------------------------
        // Examine the ShapeFactory
        //----------------------------------------------------------------------
        System.out.println(SHAPE_FACTORY_SUMMARY);

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
}
