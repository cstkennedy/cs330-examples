import java.util.ArrayList;
import java.util.List;
import java.util.Iterator;

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
     */
    public static void main(String[] args)
    {
        //----------------------------------------------------------------------
        // Print main program heading
        //----------------------------------------------------------------------
        System.out.println(projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH));

        //----------------------------------------------------------------------
        // List the available shapes
        //----------------------------------------------------------------------
        System.out.println(heading("Available Shapes", H_WIDTH, '*'));
        System.out.print(ShapeFactory.listKnown());
        System.out.println(horizontalLine('-', H_WIDTH));
        System.out.printf("%2d shapes available.%n", ShapeFactory.numberKnown());
        System.out.println();

        //----------------------------------------------------------------------
        // Create 5 "Random" Shapes
        //----------------------------------------------------------------------
        List<Shape> shapes = new ArrayList<Shape>();
        int         size   = 0; // original size of shapes container

        String[] shapesToCreate = {
            "Triangle",
            "Right Triangle",
            "Equilateral Triangle",
            "Square",
            "Circle",
            "1337 Haxor"
        };

        for (String name : shapesToCreate) {
            Shape toAdd = ShapeFactory.createShape(name);
            if (toAdd != null) {
                shapes.add(toAdd);
            }
        }

        size = shapes.size();

        System.out.println(heading("Shapes That Exist", H_WIDTH, '*'));
        System.out.printf("%-24s: %4d\n", "Original Size", size);
        System.out.printf("%-24s: %4d\n", "Invalid Shapes", (size - shapes.size()));
        System.out.printf("%-24s: %4d\n", "New Size", shapes.size());
        System.out.println();

        //----------------------------------------------------------------------
        // Print all the shapes
        //----------------------------------------------------------------------
        System.out.println(heading("Display All Shapes", H_WIDTH, '*'));

        for (Shape s : shapes) {
            System.out.println(s);
        }

        //----------------------------------------------------------------------
        // Using an iterator
        //----------------------------------------------------------------------
        System.out.println(horizontalLine('~', H_WIDTH));
        System.out.println();

        // C++ Container<Shape*>::iterator it = shapes.begin()
        Iterator<Shape> it = shapes.iterator();

        // C++ while (it != shapes.end())
        while (it.hasNext()) {
            Shape s = it.next();
            System.out.println(s);
            // System.out.println(it.next());
        }
    }
}
